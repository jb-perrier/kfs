use alloc::{boxed::Box, string::String, vec::Vec};
use char::ShellChar;

use crate::{
    keyboard::layouts::Key,
    vga::{self, encode, Colors},
};

pub mod char;
pub mod style;

#[derive(Clone, Copy)]
pub struct Cursor {
    line: usize,
    col: usize,
}

pub struct Shell {
    cmd_history: Vec<String>,
    lines: Vec<[ShellChar; vga::VGA_COLUMNS]>,
    cursor: Cursor,
    cmd_start: Cursor,
}

impl Shell {
    pub fn new() -> Self {
        Self {
            cmd_history: Vec::new(),
            lines: Vec::new(),
            cursor: Cursor { line: 0, col: 0 },
            cmd_start: Cursor { line: 0, col: 0 },
        }
    }

    pub fn start(&mut self) {
        self.lines.push([ShellChar::empty(); 80]);
        self.head();
    }

    fn head(&mut self) {
        self.lines[self.cursor.line][self.cursor.col] = ShellChar::new('>');
        self.incr_cursor();
    }

    pub fn key(&mut self, key: Key) {
        match key {
            Key::Char(c) => {
                self.lines[self.cursor.line][self.cursor.col] = ShellChar::new(c);
                self.incr_cursor();
            },
            Key::Enter => {
                let cmd = self.collect_cmd();
                self.cmd_history.push(cmd);
                self.new_line_cursor();
                self.head();
            }
            Key::Backspace => {
                self.decr_cursor();
                self.lines[self.cursor.line][self.cursor.col] = ShellChar::empty();
            }
            _ => {}
        }
    }

    pub fn log(&mut self, str: &str) {
        // TODO: parse ANSI color codes
        self.draw();
    }

    pub fn draw(&self) {
        if self.lines.len() >= vga::VGA_ROWS {
            for i in 0..vga::VGA_ROWS {
                for j in 0..vga::VGA_COLUMNS {
                    let cha = self.lines[i][j];
                    vga::write(cha.char(), i as u8, j as u8);
                }
            }
        } else {
            for i in 0..self.lines.len() {
                for j in 0..vga::VGA_COLUMNS {
                    let cha = self.lines[i][j];
                    vga::write(cha.char(), i as u8, j as u8);
                }
            }
        }
    }

    fn collect_cmd(&mut self) -> String {
        let mut cmd = String::new();
        let mut cursor = self.cmd_start;
        while cursor.line < self.cursor.line {
            let line = &self.lines[cursor.line];
            let mut col = cursor.col;
            while col < vga::VGA_COLUMNS {
                cmd.push(line[col].char());
                col += 1;
            }
            cursor.line += 1;
            cursor.col = 0;
        }
        let line = &self.lines[cursor.line];
        while cursor.col < self.cursor.col {
            cmd.push(line[cursor.col].char());
            cursor.col += 1;
        }
        cmd
    }

    fn new_line_cursor(&mut self) {
        let cursor = &mut self.cursor;
        let lines = &mut self.lines;
        cursor.line = lines.len() - 1;
        cursor.col = 0;
        lines.push([ShellChar::empty(); 80]);
    }

    fn incr_cursor(&mut self) {
        let cursor = &mut self.cursor;
        let lines = &mut self.lines;
        cursor.col += 1;
        if cursor.col == 80 {
            cursor.line += 1;
            cursor.col = 0;
            if cursor.line == lines.len() {
                lines.push([ShellChar::empty(); 80]);
            }
        }
    }

    fn decr_cursor(&mut self) {
        let cursor = &mut self.cursor;
        if cursor.col == 0 {
            if cursor.line == 0 {
                return;
            }
            cursor.line -= 1;
            cursor.col = 79;
        } else {
            cursor.col -= 1;
        }
    }
}
