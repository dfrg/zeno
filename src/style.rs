//! Path styles.

pub use super::fill::Fill;
pub use super::stroke::{Cap, Join, Stroke};

/// Represents the style of a path for rendering or hit testing.
#[derive(Copy, Clone, Debug)]
pub enum Style<'a> {
    Fill(Fill),
    Stroke(Stroke<'a>),
}

impl Default for Style<'_> {
    fn default() -> Self {
        Self::Fill(Fill::NonZero)
    }
}

impl Style<'_> {
    /// Returns true if the style is a stroke.
    pub fn is_stroke(&self) -> bool {
        match self {
            Self::Stroke(_) => true,
            _ => false,
        }
    }
}

impl From<Fill> for Style<'_> {
    fn from(style: Fill) -> Self {
        Self::Fill(style)
    }
}

impl<'a> From<Stroke<'a>> for Style<'a> {
    fn from(style: Stroke<'a>) -> Self {
        Self::Stroke(style)
    }
}

impl<'a> From<&'a Stroke<'a>> for Style<'a> {
    fn from(style: &'a Stroke<'a>) -> Self {
        Self::Stroke(*style)
    }
}

impl<'a> From<&'a mut Stroke<'a>> for Style<'a> {
    fn from(style: &'a mut Stroke<'a>) -> Self {
        Self::Stroke(*style)
    }
}
