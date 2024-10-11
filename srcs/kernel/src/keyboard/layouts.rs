
pub enum Key {
	None,
	Shift,
	Ctrl,
	CapsLock,
	LeftArrow,
	RightArrow,
	Char(char),
}
// 14 == '\b'
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
	Key::Char('b'),           /* backspace TODO */
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
	Key::Char('\n'),           /* enter */
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
	Key::None,              /* up arrow */
	Key::None,              /* PAGEUP */
	Key::Char('-'),
	Key::LeftArrow, /* left arrow */
	Key::None,
	Key::RightArrow,/* right arrow */
	Key::Char('+'),
	Key::None,              /* END */
	Key::None,              /* down arrow */
	Key::None,              /* PAGEDOWN */
	Key::None,              /* INSERT */
	Key::None,              /* DEL */
	Key::None,
	Key::None,
	Key::None,
	Key::None,              /* F11 */
	Key::None,              /* F12 */
	Key::None 
];

// pub const QWERTY_SHIFT_MAP_CONST: [Option<char>; 128] = [
//     None, None, Some('!'), Some('@'), Some('#'), Some('$'), Some('%'), Some('^'), Some('&'), Some('*'),
//     Some('('), Some(')'), Some('_'), Some('+'), None, None, Some('Q'), Some('W'), Some('E'), Some('R'),
//     Some('T'), Some('Y'), Some('U'), Some('I'), Some('O'), Some('P'), Some('{'), Some('}'), Some('\n'), None,
//     Some('A'), Some('S'), Some('D'), Some('F'), Some('G'), Some('H'), Some('J'), Some('K'), Some('L'), Some(':'),
//     Some('\"'), Some('~'), None, Some('|'), Some('Z'), Some('X'), Some('C'), Some('V'), Some('B'), Some('N'),
//     Some('M'), Some('<'), Some('>'), Some('?'), None, Some('*'), None, Some(' '), None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, Some('-'), None, None, None, Some('+'), None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None
// ];

// pub const AZERTY_MAP_CONST: [Option<char>; 128] = [
//     None, None, Some('&'), Some('é'), Some('"'), Some('\''), Some('('), Some('-'), Some('è'), Some('_'),
//     Some('ç'), Some('à'), Some(')'), Some('='), None, Some('\t'), Some('a'), Some('z'), Some('e'), Some('r'),
//     Some('t'), Some('y'), Some('u'), Some('i'), Some('o'), Some('p'), Some('^'), Some('$'), Some('\n'), None,
//     Some('q'), Some('s'), Some('d'), Some('f'), Some('g'), Some('h'), Some('j'), Some('k'), Some('l'), Some('m'),
//     Some('ù'), Some('²'), None, Some('*'), Some('w'), Some('x'), Some('c'), Some('v'), Some('b'), Some('n'),
//     Some(','), Some(';'), Some(':'), Some('!'), None, Some('*'), None, Some(' '), None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, Some('-'), None, None, None, Some('+'), None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None
// ];

// pub const AZERTY_SHIFT_MAP_CONST: [Option<char>; 128] = [
//     None, None, Some('1'), Some('2'), Some('3'), Some('4'), Some('5'), Some('6'), Some('7'), Some('8'),
//     Some('9'), Some('0'), None, Some('_'), None, None, Some('A'), Some('Z'), Some('E'), Some('R'),
//     Some('T'), Some('Y'), Some('U'), Some('I'), Some('O'), Some('P'), None, Some('*'), Some('\n'), None,
//     Some('Q'), Some('S'), Some('D'), Some('F'), Some('G'), Some('H'), Some('J'), Some('K'), Some('L'), Some('M'),
//     Some('%'), Some('>'), None, None, Some('W'), Some('X'), Some('C'), Some('V'), Some('B'), Some('N'),
//     Some('?'), Some('.'), Some('/'), Some('+'), None, Some('*'), None, Some(' '), None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, Some('-'), None, None, None, Some('+'), None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None,
//     None, None, None, None, None, None, None, None, None, None, None, None
// ];

pub fn get_char(layout: &[Key; 128], scancode: u8) -> Key {
	layout.get(scancode).unwrap_or(Key::None)
}