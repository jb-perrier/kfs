use super::Key;

pub const QWERTY_MAP: [Key; 128] = [
	Key::None,
    Key::None,
    Key::Char('1'),
    Key::Char('2'),
    Key::Char('3'),
    Key::Char('4'),
    Key::Char('5'),
    Key::Char('6'),
    Key::Char('7'),
    Key::Char('8'),
    Key::Char('9'),
    Key::Char('0'),
    Key::Char('-'),
	Key::Char('='),
	Key::Backspace,           /* backspace */
	Key::Char('\t'),           /* tabulation */
    Key::Char('q'),
	Key::Char('w'),
	Key::Char('e'),
	Key::Char('r'),
	Key::Char('t'),
	Key::Char('y'),
	Key::Char('u'),
	Key::Char('i'),
	Key::Char('o'),
	Key::Char('p'),
	Key::Char('['),
	Key::Char(']'),
	Key::Enter,           /* enter */
	Key::Ctrl,       /* control */
	Key::Char('a'),
    Key::Char('s'),
	Key::Char('d'),
	Key::Char('f'),
	Key::Char('g'),
	Key::Char('h'),
	Key::Char('j'),
	Key::Char('k'),
	Key::Char('l'),
	Key::Char(';'),
	Key::Char('\''),
	Key::Char('`'),
	Key::Shift,       /* left shift */
	Key::Char('\\'),
	Key::Char('z'),
	Key::Char('x'),
    Key::Char('c'),
	Key::Char('v'),
	Key::Char('b'),
	Key::Char('n'),
	Key::Char('m'),
	Key::Char(','),
	Key::Char('.'),
	Key::Char('/'),
	Key::Shift,       /* right shift */
	Key::Char('*'),
	Key::None,              /* alt */
	Key::Char(' '),            /* space */
	Key::CapsLock,   /* caps lock */
	Key::None,              /* F1 */
	Key::None,              /* F2 */
	Key::None,              /* F3 */
	Key::None,              /* F4 */
	Key::None,              /* F5 */
    Key::None,              /* F6 */
	Key::None,              /* F7 */
	Key::None,              /* F8 */
	Key::None,              /* F9 */
	Key::None,              /* F10 */
	Key::None,              /* num lock */
	Key::None,              /* scroll lock */
	Key::None,              /* HOME */
	Key::UpArrow,              /* up arrow */
	Key::None,              /* PAGEUP */
	Key::Char('-'),
	Key::LeftArrow, /* left arrow */
	Key::None,
	Key::RightArrow,/* right arrow */
	Key::Char('+'),
	Key::None,              /* END */
	Key::DownArrow,              /* down arrow */
	Key::None,              /* PAGEDOWN */
	Key::None,              /* INSERT */
	Key::None,              /* DEL */
	Key::None,
	Key::None,
	Key::None,
	Key::None,              /* F11 */
	Key::None,              /* F12 */
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
];

pub const QWERTY_MAP_MAJ: [Key; 128] = [
	Key::None,
    Key::None,
    Key::Char('!'),
    Key::Char('@'),
    Key::Char('#'),
    Key::Char('$'),
    Key::Char('%'),
    Key::Char('^'),
    Key::Char('&'),
    Key::Char('*'),
    Key::Char('('),
    Key::Char(')'),
    Key::Char('_'),
	Key::Char('+'),
	Key::Backspace,           /* backspace */
	Key::Char('\t'),           /* tabulation */
    Key::Char('Q'),
	Key::Char('W'),
	Key::Char('E'),
	Key::Char('R'),
	Key::Char('T'),
	Key::Char('Y'),
	Key::Char('U'),
	Key::Char('I'),
	Key::Char('O'),
	Key::Char('P'),
	Key::Char('{'),
	Key::Char('}'),
	Key::Enter,           /* enter */
	Key::Ctrl,       /* control */
	Key::Char('A'),
    Key::Char('S'),
	Key::Char('D'),
	Key::Char('F'),
	Key::Char('G'),
	Key::Char('H'),
	Key::Char('J'),
	Key::Char('K'),
	Key::Char('L'),
	Key::Char(':'),
	Key::Char('"'),
	Key::Char('~'),
	Key::Shift,       /* left shift */
	Key::Char('|'),
	Key::Char('Z'),
	Key::Char('X'),
    Key::Char('C'),
	Key::Char('V'),
	Key::Char('B'),
	Key::Char('N'),
	Key::Char('M'),
	Key::Char('<'),
	Key::Char('>'),
	Key::Char('?'),
	Key::Shift,       /* right shift */
	Key::Char('*'),
	Key::None,              /* alt */
	Key::Char(' '),            /* space */
	Key::CapsLock,   /* caps lock */
	Key::None,              /* F1 */
	Key::None,              /* F2 */
	Key::None,              /* F3 */
	Key::None,              /* F4 */
	Key::None,              /* F5 */
    Key::None,              /* F6 */
	Key::None,              /* F7 */
	Key::None,              /* F8 */
	Key::None,              /* F9 */
	Key::None,              /* F10 */
	Key::None,              /* num lock */
	Key::None,              /* scroll lock */
	Key::None,              /* HOME */
	Key::UpArrow,              /* up arrow */
	Key::None,              /* PAGEUP */
	Key::Char('-'),
	Key::LeftArrow, /* left arrow */
	Key::None,
	Key::RightArrow,/* right arrow */
	Key::Char('+'),
	Key::None,              /* END */
	Key::DownArrow,              /* down arrow */
	Key::None,              /* PAGEDOWN */
	Key::None,              /* INSERT */
	Key::None,              /* DEL */
	Key::None,
	Key::None,
	Key::None,
	Key::None,              /* F11 */
	Key::None,              /* F12 */
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
	Key::None,
];