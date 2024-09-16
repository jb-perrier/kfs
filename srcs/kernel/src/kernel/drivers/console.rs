pub trait Console {
    fn get_size() -> (usize, usize);
    fn set_cursor_pos(pos: (usize, usize)) -> Result<(), String>;
    fn get_cursor_pos() -> (usize, usize);
    fn write(c: char, pos: (usize, usize));
    // return character + foreground color + background color
    fn read(pos: (usize, usize)) -> (char, u8, u8);
    fn clear();
}