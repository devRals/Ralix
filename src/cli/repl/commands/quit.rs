use super::Repl;
pub(super) fn run(repl: &mut Repl, _arguments: &[String]) {
    repl.should_quit = true;
}

pub(super) fn content() -> &'static str {
    "Quit's from the REPL"
}
