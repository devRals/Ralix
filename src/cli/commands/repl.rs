use std::io::{self, BufReader, stderr, stdin, stdout};

use crate::repl::Repl;

pub fn run() -> io::Result<()> {
    let mut repl = Repl::new(BufReader::new(stdin()), stdout(), stderr());
    repl.run()
}
