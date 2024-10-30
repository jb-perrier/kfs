use crate::vga::{self, encode, Colors};


#[derive(Clone, Copy)]
pub struct ShellChar(u16);

impl ShellChar {
    pub fn new(c: char) -> Self {
        let c = encode(c, Colors::White, Colors::Black);
        Self(c)
    }

    pub fn new_with_colors(c: char, fore_color: Colors, back_color: Colors) -> Self {
        let c = encode(c, fore_color, back_color);
        Self(c)
    }

    pub fn empty() -> Self {
        Self(vga::empty_char())
    }

    pub fn char(&self) -> char {
        let (c, _, _) = vga::decode(self.0);
        c
    }

    pub fn fore_color(&self) -> Colors {
        let (_, fore_color, _) = vga::decode(self.0);
        fore_color
    }

    pub fn back_color(&self) -> Colors {
        let (_, _, back_color) = vga::decode(self.0);
        back_color
    }
}