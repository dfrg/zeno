# zeno

Zeno is a pure Rust crate that provides a high performance, low level 2D 
rasterization library with support for rendering paths of various styles 
into alpha or subpixel masks.

[![Crates.io][crates-badge]][crates-url]
[![Docs.rs][docs-badge]][docs-url]
[![MIT licensed][mit-badge]][mit-url]
[![Apache licensed][apache-badge]][apache-url]

[crates-badge]: https://img.shields.io/crates/v/zeno.svg
[crates-url]: https://crates.io/crates/zeno
[docs-badge]: https://docs.rs/zeno/badge.svg
[docs-url]: https://docs.rs/zeno
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/license-Apache--2.0-blue.svg
[apache-url]: LICENSE-APACHE

## Features

- 256x anti-aliased rasterization (8-bit alpha or 32-bit RGBA subpixel alpha)
- Pixel perfect hit testing with customizable coverage threshold
- Non-zero and even-odd fills
- Stroking with the standard set of joins and caps
    (separate start and end caps are possible)
- Numerically stable dashing for smooth dash offset animation
- Vertex traversal for marker placement
- Stepped distance traversal for animation or text-on-path support
- Abstract representation of path data that imposes no policy on storage

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
zeno = "0.2.1"
```

Rendering a dashed stroke of a triangle:

```rust
use zeno::{Cap, Join, Mask, PathData, Stroke};

// Buffer to store the mask
let mut mask = [0u8; 64 * 64];

/// Create a mask builder with some path data
Mask::new("M 8,56 32,8 56,56 Z")
    .style(
        // Stroke style with a width of 4
        Stroke::new(4.0)
            // Round line joins
            .join(Join::Round)
            // And round line caps
            .cap(Cap::Round)
            // Dash pattern followed by a dash offset
            .dash(&[10.0, 12.0, 0.0], 0.0),
    )
    // Set the target dimensions
    .size(64, 64)
    // Render into the target buffer
    .render_into(&mut mask, None);
```

Resulting in the following mask: 

![Dashed Triangle](https://muddl.com/zeno/tri_dash.png)

For detail on additional features and more advanced usage,
see the full API [documentation](https://docs.rs/zeno).