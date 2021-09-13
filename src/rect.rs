use crate::vec2::*;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {x: x, y: y, w: w, h: h}
    }
    pub fn new_centered(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(x-w/2.0, y-h/2.0, w, h)
    }

    pub fn dilate(&self, d: f32) -> Rect {
        return Rect::new(self.x - d, self.y - d, self.w + 2.0*d, self.h + 2.0*d);
    }

    pub fn left(self) -> f32 {
        self.x
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn top(self) -> f32 {
        self.y
    }
    pub fn bot(self) -> f32 {
        self.y + self.h
    }
    pub fn center(self) -> Vec2  {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }
}

#[test]
fn test_intersection() {
    assert_eq!(rect_intersection(Rect::new(0.0, 0.0, 1.0, 1.0,), Rect::new(0.5, 0.0, 1.0, 1.0)), true);
    assert_eq!(rect_intersection(Rect::new(0.0, 0.0, 1.0, 1.0,), Rect::new(-0.5, 0.0, 1.0, 1.0)), true);
    assert_eq!(rect_intersection(Rect::new(0.0, 0.0, 1.0, 1.0,), Rect::new(0.0, 0.5, 1.0, 1.0)), true);
    assert_eq!(rect_intersection(Rect::new(0.0, 0.0, 1.0, 1.0,), Rect::new(0.0, -0.5, 1.0, 1.0)), true);

    assert_eq!(rect_intersection(Rect::new(0.0, 0.0, 1.0, 1.0,), Rect::new(0.5, -0.05, 0.1, 0.1)), true);
}

// not overlapping if sides kiss
fn overlap_1d(a1: f32, a2: f32, b1: f32, b2: f32) -> bool {
    (b1 > a1 && b1 < a2) ||
    (a1 > b1 && a1 < b2) ||
    a1 == b1 && a2 == b2
}

pub fn rect_intersection(a: Rect, b: Rect) -> bool {
    overlap_1d(a.left(), a.right(), b.left(), b.right()) &&
    overlap_1d(a.top(), a.bot(), b.top(), b.bot())
}



