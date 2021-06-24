/// Trait representing basic operations on a timestamp value.
/// Timestamps are often used in lock-free protocols to implement
/// versioned values.
pub trait Timestamp {
    /// Create a new timestamp.
    fn new() -> Self;

    /// Check `self` is earlier than `other`.
    fn earlier(&self, other: &Self) -> bool;

    /// Check `self` is later than `other`.
    fn later(&self, other: &Self) -> bool;

    /// Get the successor of this timestamp.
    fn succ(&self) -> Self;
}
