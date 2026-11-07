//! Buffer size computation for large images must not wrap.
//!
//! Discovered through a crafted font whose rasterization bounds made
//! `width * height` overflow `u32` arithmetic (dfrg/swash#124).

use zeno::Format;

#[test]
fn buffer_size_small_image() {
    assert_eq!(Format::Alpha.buffer_size(16, 16), 256);
    assert_eq!(Format::Subpixel.buffer_size(16, 16), 1024);
}

#[cfg(target_pointer_width = "64")]
#[test]
fn buffer_size_does_not_truncate_at_32_bits() {
    // 65536 * 65536 == 2^32 overflows `u32`, wrapping to 0 before this fix.
    assert_eq!(Format::Alpha.buffer_size(65536, 65536), 1usize << 32);
    assert_eq!(Format::Subpixel.buffer_size(65536, 65536), 1usize << 34);
}

#[test]
fn buffer_size_saturates_rather_than_wrapping() {
    assert_eq!(Format::Subpixel.buffer_size(u32::MAX, u32::MAX), usize::MAX);
}
