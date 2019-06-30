use termion::color;

pub struct Entity {
    pub x: i64,
    pub y: i64,
    pub c: char,
    pub color: color::Rgb,
}

impl Entity {
    pub fn new(x: i64, y: i64, c: char, color: color::Rgb) -> Self {
        Self { x, y, c, color }
    }

    pub fn mov(&mut self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }
}
