use std::{fmt::Display, io};

pub struct Logger<W: io::Write> {
    writer: W,
}

const ERROR_PREFIX: &str = "\x1b[1;31mError\x1b[0m: ";
const INFO_PREFIX: &str = "\x1b[1;34mInfo\x1b[0m: ";
const WARN_PREFIX: &str = "\x1b[1;33mWarning\x1b[0m: ";

impl<W: io::Write> Logger<W> {
    pub const fn new(writer: W) -> Logger<W> {
        Logger { writer }
    }

    pub fn info<D: Display>(&mut self, msg: D) -> io::Result<()> {
        writeln!(self.writer, "{INFO_PREFIX}{msg}")
    }

    pub fn warn<D: Display>(&mut self, msg: D) -> io::Result<()> {
        writeln!(self.writer, "{WARN_PREFIX}{msg}")
    }

    pub fn error<D: Display>(&mut self, msg: D) -> io::Result<()> {
        writeln!(self.writer, "{ERROR_PREFIX}{msg}")
    }
}
