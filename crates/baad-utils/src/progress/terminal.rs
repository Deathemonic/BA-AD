use std::env::var;
use std::io::{self, IsTerminal};

use terminal_size::terminal_size;

pub fn is_terminal() -> bool {
    io::stderr().is_terminal() && !var("TERM").is_ok_and(|term| term.eq_ignore_ascii_case("dumb"))
}

pub fn width() -> Option<usize> { terminal_size().map(|(w, _)| w.0 as usize) }
