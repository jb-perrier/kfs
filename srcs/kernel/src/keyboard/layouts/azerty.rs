use super::Key;

pub const AZERTY_MAP: [Key; 128] = [
    /* 0 */ Key::None,
    /* 1 */ Key::None,
    /* 2 */ Key::Char('1'),
    /* 3 */ Key::Char('2'),
    /* 4 */ Key::Char('3'),
    /* 5 */ Key::Char('4'),
    /* 6 */ Key::Char('5'),
    /* 7 */ Key::Char('6'),
    /* 8 */ Key::Char('7'),
    /* 9 */ Key::Char('8'),
    /* 10 */ Key::Char('9'),
    /* 11 */ Key::Char('0'),
    /* 12 */ Key::Char('-'),
    /* 13 */ Key::Char('='),
    /* 14 */ Key::Backspace,           /* backspace */
    /* 15 */ Key::Char('\t'),           /* tabulation */
    /* 16 */ Key::Char('a'),
    /* 17 */ Key::Char('z'),
    /* 18 */ Key::Char('e'),
    /* 19 */ Key::Char('r'),
    /* 20 */ Key::Char('t'),
    /* 21 */ Key::Char('y'),
    /* 22 */ Key::Char('u'),
    /* 23 */ Key::Char('i'),
    /* 24 */ Key::Char('o'),
    /* 25 */ Key::Char('p'),
    /* 26 */ Key::Char('['),
    /* 27 */ Key::Char(']'),
    /* 28 */ Key::Enter,           /* enter */
    /* 29 */ Key::Ctrl,       /* control */
    /* 30 */ Key::Char('q'),
    /* 31 */ Key::Char('s'),
    /* 32 */ Key::Char('d'),
    /* 33 */ Key::Char('f'),
    /* 34 */ Key::Char('g'),
    /* 35 */ Key::Char('h'),
    /* 36 */ Key::Char('j'),
    /* 37 */ Key::Char('k'),
    /* 38 */ Key::Char('l'),
    /* 39 */ Key::Char('m'),
    /* 40 */ Key::Char('\''),
    /* 41 */ Key::Char('`'),
    /* 42 */ Key::Shift,       /* left shift */
    /* 43 */ Key::Char('*'),
    /* 44 */ Key::Char('w'),
    /* 45 */ Key::Char('x'),
    /* 46 */ Key::Char('c'),
    /* 47 */ Key::Char('v'),
    /* 48 */ Key::Char('b'),
    /* 49 */ Key::Char('n'),
    /* 50 */ Key::Char(','),
    /* 51 */ Key::Char(';'),
    /* 52 */ Key::Char(':'),
    /* 53 */ Key::Char('!'),
    /* 54 */ Key::Shift,       /* right shift */
    /* 55 */ Key::Char('*'),
    /* 56 */ Key::None,              /* alt */
    /* 57 */ Key::Char(' '),            /* space */
    /* 58 */ Key::CapsLock,   /* caps lock */
    /* 59 */ Key::None,              /* F1 */
    /* 60 */ Key::None,              /* F2 */
    /* 61 */ Key::None,              /* F3 */
    /* 62 */ Key::None,              /* F4 */
    /* 63 */ Key::None,              /* F5 */
    /* 64 */ Key::None,              /* F6 */
    /* 65 */ Key::None,              /* F7 */
    /* 66 */ Key::None,              /* F8 */
    /* 67 */ Key::None,              /* F9 */
    /* 68 */ Key::None,              /* F10 */
    /* 69 */ Key::None,              /* num lock */
    /* 70 */ Key::None,              /* scroll lock */
    /* 71 */ Key::None,              /* HOME */
    /* 72 */ Key::UpArrow,              /* up arrow */
    /* 73 */ Key::None,              /* PAGEUP */
    /* 74 */ Key::Char('-'),
    /* 75 */ Key::LeftArrow, /* left arrow */
    /* 76 */ Key::None,
    /* 77 */ Key::RightArrow,/* right arrow */
    /* 78 */ Key::Char('+'),
    /* 79 */ Key::None,              /* END */
    /* 80 */ Key::DownArrow,              /* down arrow */
    /* 81 */ Key::None,              /* PAGEDOWN */
    /* 82 */ Key::None,              /* INSERT */
    /* 83 */ Key::None,              /* DEL */
    /* 84 */ Key::None,
    /* 85 */ Key::None,
    /* 86 */ Key::Char('<'),
    /* 87 */ Key::None,              /* F11 */
    /* 88 */ Key::None,              /* F12 */
    /* 89 */ Key::None,
    /* 90 */ Key::None,
    /* 91 */ Key::None,
    /* 92 */ Key::None,
    /* 93 */ Key::None,
    /* 94 */ Key::None,
    /* 95 */ Key::None,
    /* 96 */ Key::None,
    /* 97 */ Key::None,
    /* 98 */ Key::None,
    /* 99 */ Key::None,
    /* 100 */ Key::None,
    /* 101 */ Key::None,
    /* 102 */ Key::None,
    /* 103 */ Key::None,
    /* 104 */ Key::None,
    /* 105 */ Key::None,
    /* 106 */ Key::None,
    /* 107 */ Key::None,
    /* 108 */ Key::None,
    /* 109 */ Key::None,
    /* 110 */ Key::None,
    /* 111 */ Key::None,
    /* 112 */ Key::None,
    /* 113 */ Key::None,
    /* 114 */ Key::None,
    /* 115 */ Key::None,
    /* 116 */ Key::None,
    /* 117 */ Key::None,
    /* 118 */ Key::None,
    /* 119 */ Key::None,
    /* 120 */ Key::None,
    /* 121 */ Key::None,
    /* 122 */ Key::None,
    /* 123 */ Key::None,
    /* 124 */ Key::None,
    /* 125 */ Key::None,
    /* 126 */ Key::None,
    /* 127 */ Key::None,
];

pub const AZERTY_MAP_MAJ: [Key; 128] = [
    /* 0 */ Key::None,
    /* 1 */ Key::None,
    /* 2 */ Key::Char('!'),
    /* 3 */ Key::Char('@'),
    /* 4 */ Key::Char('#'),
    /* 5 */ Key::Char('$'),
    /* 6 */ Key::Char('%'),
    /* 7 */ Key::Char('^'),
    /* 8 */ Key::Char('&'),
    /* 9 */ Key::Char('*'),
    /* 10 */ Key::Char('('),
    /* 11 */ Key::Char(')'),
    /* 12 */ Key::Char('_'),
    /* 13 */ Key::Char('+'),
    /* 14 */ Key::Backspace,           /* backspace */
    /* 15 */ Key::Char('\t'),           /* tabulation */
    /* 16 */ Key::Char('A'),
    /* 17 */ Key::Char('Z'),
    /* 18 */ Key::Char('E'),
    /* 19 */ Key::Char('R'),
    /* 20 */ Key::Char('T'),
    /* 21 */ Key::Char('Y'),
    /* 22 */ Key::Char('U'),
    /* 23 */ Key::Char('I'),
    /* 24 */ Key::Char('O'),
    /* 25 */ Key::Char('P'),
    /* 26 */ Key::Char('{'),
    /* 27 */ Key::Char('}'),
    /* 28 */ Key::Enter,           /* enter */
    /* 29 */ Key::Ctrl,       /* control */
    /* 30 */ Key::Char('Q'),
    /* 31 */ Key::Char('S'),
    /* 32 */ Key::Char('D'),
    /* 33 */ Key::Char('F'),
    /* 34 */ Key::Char('G'),
    /* 35 */ Key::Char('H'),
    /* 36 */ Key::Char('J'),
    /* 37 */ Key::Char('K'),
    /* 38 */ Key::Char('L'),
    /* 39 */ Key::Char('M'),
    /* 40 */ Key::Char('"'),
    /* 41 */ Key::Char('~'),
    /* 42 */ Key::Shift,       /* left shift */
    /* 43 */ Key::Char('|'),
    /* 44 */ Key::Char('W'),
    /* 45 */ Key::Char('X'),
    /* 46 */ Key::Char('C'),
    /* 47 */ Key::Char('V'),
    /* 48 */ Key::Char('B'),
    /* 49 */ Key::Char('N'),
    /* 50 */ Key::Char('?'),
    /* 51 */ Key::Char('.'),
    /* 52 */ Key::Char('/'),
    /* 53 */ Key::Char('?'),
    /* 54 */ Key::Shift,       /* right shift */
    /* 55 */ Key::Char('*'),
    /* 56 */ Key::None,              /* alt */
    /* 57 */ Key::Char(' '),            /* space */
    /* 58 */ Key::CapsLock,   /* caps lock */
    /* 59 */ Key::None,              /* F1 */
    /* 60 */ Key::None,              /* F2 */
    /* 61 */ Key::None,              /* F3 */
    /* 62 */ Key::None,              /* F4 */
    /* 63 */ Key::None,              /* F5 */
    /* 64 */ Key::None,              /* F6 */
    /* 65 */ Key::None,              /* F7 */
    /* 66 */ Key::None,              /* F8 */
    /* 67 */ Key::None,              /* F9 */
    /* 68 */ Key::None,              /* F10 */
    /* 69 */ Key::None,              /* num lock */
    /* 70 */ Key::None,              /* scroll lock */
    /* 71 */ Key::None,              /* HOME */
    /* 72 */ Key::UpArrow,              /* up arrow */
    /* 73 */ Key::None,              /* PAGEUP */
    /* 74 */ Key::Char('-'),
    /* 75 */ Key::LeftArrow, /* left arrow */
    /* 76 */ Key::None,
    /* 77 */ Key::RightArrow,/* right arrow */
    /* 78 */ Key::Char('+'),
    /* 79 */ Key::None,              /* END */
    /* 80 */ Key::DownArrow,              /* down arrow */
    /* 81 */ Key::None,              /* PAGEDOWN */
    /* 82 */ Key::None,              /* INSERT */
    /* 83 */ Key::None,              /* DEL */
    /* 84 */ Key::None,
    /* 85 */ Key::None,
    /* 86 */ Key::Char('>'),
    /* 87 */ Key::None,              /* F11 */
    /* 88 */ Key::None,              /* F12 */
    /* 89 */ Key::None,
    /* 90 */ Key::None,
    /* 91 */ Key::None,
    /* 92 */ Key::None,
    /* 93 */ Key::None,
    /* 94 */ Key::None,
    /* 95 */ Key::None,
    /* 96 */ Key::None,
    /* 97 */ Key::None,
    /* 98 */ Key::None,
    /* 99 */ Key::None,
    /* 100 */ Key::None,
    /* 101 */ Key::None,
    /* 102 */ Key::None,
    /* 103 */ Key::None,
    /* 104 */ Key::None,
    /* 105 */ Key::None,
    /* 106 */ Key::None,
    /* 107 */ Key::None,
    /* 108 */ Key::None,
    /* 109 */ Key::None,
    /* 110 */ Key::None,
    /* 111 */ Key::None,
    /* 112 */ Key::None,
    /* 113 */ Key::None,
    /* 114 */ Key::None,
    /* 115 */ Key::None,
    /* 116 */ Key::None,
    /* 117 */ Key::None,
    /* 118 */ Key::None,
    /* 119 */ Key::None,
    /* 120 */ Key::None,
    /* 121 */ Key::None,
    /* 122 */ Key::None,
    /* 123 */ Key::None,
    /* 124 */ Key::None,
    /* 125 */ Key::None,
    /* 126 */ Key::None,
    /* 127 */ Key::None,
];
