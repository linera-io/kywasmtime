use crate::sleep::Sleep;
use crate::time::Instant;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Debug)]
pub struct Interval {
    sleep: Sleep,
    period: Duration,
    missed_tick_behavior: MissedTickBehavior,
}

impl Interval {
    fn new(start: Instant, period: Duration) -> Self {
        Self {
            sleep: Sleep::new(start),
            period,
            missed_tick_behavior: MissedTickBehavior::default(),
        }
    }

    pub async fn tick(&mut self) -> Instant {
        TickFuture(self).await
    }

    pub fn poll_tick(&mut self, cx: &mut Context<'_>) -> Poll<Instant> {
        let fut = Pin::new(&mut self.sleep);
        if fut.poll(cx).is_pending() {
            return Poll::Pending;
        }

        let timeout = self.sleep.deadline();
        let now = Instant::now();

        let next = self
            .missed_tick_behavior
            .next_timeout(timeout, now, self.period);
        Pin::new(&mut self.sleep).reset(next);

        return Poll::Ready(timeout);
    }

    pub fn reset(&mut self) {
        Pin::new(&mut self.sleep).reset(Instant::now() + self.period);
    }

    pub fn missed_tick_behavior(&self) -> MissedTickBehavior {
        self.missed_tick_behavior
    }

    pub fn set_missed_tick_behavior(&mut self, behavior: MissedTickBehavior) {
        self.missed_tick_behavior = behavior;
    }
}

struct TickFuture<'a>(&'a mut Interval);

impl<'a> Future for TickFuture<'a> {
    type Output = Instant;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().0.poll_tick(cx)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum MissedTickBehavior {
    #[default]
    Burst,
    Delay,
    Skip,
}

impl MissedTickBehavior {
    fn next_timeout(&self, timeout: Instant, now: Instant, period: Duration) -> Instant {
        match self {
            Self::Burst => timeout + period,
            Self::Delay => now + period,
            Self::Skip => {
                let now_us = now.raw_us();
                let period_us: u64 = period.as_micros().try_into().unwrap();
                let timeout_us = timeout.raw_us();
                let next = now_us + period_us - ((now_us - timeout_us) % period_us);
                Instant::from_raw_us(next)
            }
        }
    }
}

pub fn interval_at(start: Instant, period: Duration) -> Interval {
    Interval::new(start, period)
}

pub fn interval(period: Duration) -> Interval {
    interval_at(Instant::now(), period)
}
