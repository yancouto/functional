use std::fmt;
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
    pub fn new(i: i32, j: i32) -> Self {
        Self { i, j }
    }

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    pub w: i32,
    pub h: i32,
}

impl Size {
    pub fn new(w: i32, h: i32) -> Self {
        Self { w, h }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rect {
    pub pos: Pos,
    pub size: Size,
}

impl Rect {
    pub fn new(i: i32, j: i32, w: i32, h: i32) -> Self {
        Self {
            pos: Pos { i, j },
            size: Size { w, h },
        }
    }
}
