use crossterm::{cursor::MoveToColumn, ExecutableCommand};
use std::io::{self, Write};

pub fn log(msg: impl AsRef<str>) {
    io::stdout().write_all(msg.as_ref().as_bytes()).unwrap();
    io::stdout().execute(MoveToColumn(0)).unwrap();
}
