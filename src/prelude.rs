//! Things useful for everyone

pub const W: i32 = 130;
pub const H: i32 = 80;
pub use std::sync::Arc;

pub use bl::VirtualKeyCode as Key;
pub use bracket_lib::prelude as bl;
pub use more_asserts::*;
pub use rayon::prelude::*;
pub use vec1::{vec1, Vec1};

pub use crate::{
    math::{Pos, Rect}, utils::debug_asserts::*
};
