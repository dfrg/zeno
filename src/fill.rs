//! Fill rule.

/// Describes the visual style of a fill.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Fill {
    /// The non-zero fill rule.
    NonZero,
    /// The even-odd fill rule.
    EvenOdd,
}
