/// Error returned by [crate::timeout][`timeout`] if the timeout
/// elapses before the future completes.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Elapsed;
