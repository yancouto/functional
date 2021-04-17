use std::{fmt, ops};

use crate::prelude::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Pos {
    pub i: i32,
    pub j: i32,
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Pos").field(&self.i).field(&self.j).finish()
    }
}

impl Pos {
    pub fn new(i: i32, j: i32) -> Self { Self { i, j } }

    pub fn from_xy(xy: (i32, i32)) -> Self { Self { i: xy.1, j: xy.0 } }

    pub fn inside(&self, r: &Rect) -> bool {
        if self.i < r.pos.i
            || self.i >= r.pos.i + r.size.h
            || self.j < r.pos.j
            || self.j >= r.pos.j + r.size.w
        {
            false
        } else {
            true
        }
    }
}

impl ops::Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { Self::new(self.i - rhs.i, self.j - rhs.j) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Size {
    pub fn new(w: i32, h: i32) -> Self { Self { w, h } }
}

/// A rectangle that's a single point is represented as a rectangle with size (1, 1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub pos:  Pos,
    pub size: Size,
}

impl Rect {
    pub fn new(i: i32, j: i32, w: i32, h: i32) -> Self {
        Self {
            pos:  Pos { i, j },
            size: Size { w, h },
        }
    }

    pub fn centered(w: i32, h: i32) -> Self {
        Self {
            pos:  Pos {
                i: H / 2 - h / 2,
                j: W / 2 - w / 2,
            },
            size: Size { w, h },
        }
    }

    pub fn bottom(&self) -> i32 { self.pos.i + self.size.h - 1 }

    pub fn left(&self) -> i32 { self.pos.j }
}
