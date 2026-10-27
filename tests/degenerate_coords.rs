//! Rendering paths with non-finite coordinates must not invoke undefined
//! behavior.

use zeno::{Command, Mask, Point};

fn render_with_line_to(x: f32, y: f32) {
    let path = vec![
        Command::MoveTo(Point::new(0., 0.)),
        Command::LineTo(Point::new(x, y)),
        Command::LineTo(Point::new(16., 32.)),
        Command::Close,
    ];
    let mut buffer = [0u8; 64 * 64];
    Mask::new(&path).size(64, 64).render_into(&mut buffer, None);
}

#[test]
fn nan_coords() {
    render_with_line_to(f32::NAN, 8.);
    render_with_line_to(8., f32::NAN);
    render_with_line_to(f32::NAN, f32::NAN);
}
