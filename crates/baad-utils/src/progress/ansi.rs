use std::io::Write;

pub const MOVE_TO_START: &[u8] = b"\x1b[1G";
pub const CLEAR_TO_END_OF_SCREEN: &[u8] = b"\x1b[0J";
pub const CLEAR_TO_END_OF_LINE: &[u8] = b"\x1b[0K";
pub const DISABLE_LINE_WRAP: &[u8] = b"\x1b[?7l";
pub const ENABLE_LINE_WRAP: &[u8] = b"\x1b[?7h";

pub fn cursor_up_and_home(lines: usize, buffer: &mut Vec<u8>) {
    if lines > 0 {
        let _ = write!(buffer, "\x1b[{lines}F");
    } else {
        buffer.extend_from_slice(MOVE_TO_START);
    }
}
