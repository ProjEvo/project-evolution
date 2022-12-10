//! Stores generic util methods that don't really belong in a specific module

use crate::simulation::{WORLD_X_SIZE, WORLD_Y_SIZE};
use std::{
    cmp::Ordering,
    ops::{Bound, RangeBounds},
};

const MAX_RGB: u8 = 255;

/// Takes in hsv and outputs rgb, where:
/// h: [0, 360], s: [0, 100], v: [0, 100]
/// r: [0, 255], g: [0, 255], b: [0, 255]
pub fn hsv_to_rgb(h: u16, s: u8, v: u8) -> (u8, u8, u8) {
    let delta_s = s as f32 / 100.0;
    let delta_v = v as f32 / 100.0;

    let i = ((h % 360) / 60) as u8;
    let dh = ((h % 360) as f32 / 60.0) - i as f32;
    let rv = (delta_v * MAX_RGB as f32) as u8;

    if s == 0 {
        return (rv, rv, rv);
    }

    let p = (delta_v * (1.0 - delta_s) * 255.0) as u8;
    let q = (delta_v * (1.0 - (delta_s * dh)) * 255.0) as u8;
    let t = (delta_v * (1.0 - (delta_s * (1.0 - dh))) * 255.0) as u8;

    match i {
        0 => (rv, t, p),
        1 => (q, rv, p),
        2 => (p, rv, t),
        3 => (p, q, rv),
        4 => (t, p, rv),
        _ => (rv, p, q),
    }
}

/// Gets the distance between two vectors
pub fn distance(a: &rapier::prelude::Vector<f32>, b: &rapier::prelude::Vector<f32>) -> f32 {
    f32::sqrt(f32::powi(a.x - b.x, 2) + f32::powi(a.y - b.y, 2))
}

/// Converts world x to screen x
pub fn transform_x_from_world_to_screen(x: f32, screen_size: &egui::Vec2) -> f32 {
    let x_factor = screen_size.x / WORLD_X_SIZE;

    x * x_factor
}

/// Converts world y to screen y
pub fn transform_y_from_world_to_screen(y: f32, screen_size: &egui::Vec2) -> f32 {
    let y_factor = screen_size.y / WORLD_Y_SIZE;

    y * y_factor
}

/// Converts physics coordinates to screen coordinates
pub fn transform_position_from_world_to_screen_pos2(
    position: &rapier::prelude::Vector<f32>,
    screen_size: &egui::Vec2,
) -> egui::Pos2 {
    egui::Pos2 {
        x: transform_x_from_world_to_screen(position.x, screen_size),
        y: transform_y_from_world_to_screen(position.y, screen_size),
    }
}

/// Compares two f32s a and b
///
/// # Panics
/// When a or b is NAN
pub fn cmp_f32(a: &f32, b: &f32) -> Ordering {
    a.partial_cmp(b).expect("NAN!")
}

/// Clamps a number to a range
///
/// # Panics
/// - If NAN is involved
/// - If the range isn't bounded on both sides
pub fn clamp_to_range<T: PartialOrd<T> + Clone, R: RangeBounds<T>>(input: T, range: R) -> T {
    match range.start_bound() {
        Bound::Included(start) | Bound::Excluded(start) => {
            if input.partial_cmp(start).expect("Cannot clamp NAN") == Ordering::Less {
                return start.clone();
            }
        }
        Bound::Unbounded => panic!("Must be bounded!"),
    }

    match range.end_bound() {
        Bound::Included(end) | Bound::Excluded(end) => {
            if input.partial_cmp(end).expect("Cannot clamp NAN") == Ordering::Greater {
                return end.clone();
            }
        }
        Bound::Unbounded => panic!("Must be bounded!"),
    }

    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn hsv_to_rgb_test() {
        assert_eq!(hsv_to_rgb(0, 100, 100), (255, 0, 0));
        assert_eq!(hsv_to_rgb(0, 50, 100), (255, 127, 127));
        assert_eq!(hsv_to_rgb(67, 65, 34), (80, 86, 30));
        assert_eq!(hsv_to_rgb(236, 66, 63), (54, 61, 160));
    }
}
