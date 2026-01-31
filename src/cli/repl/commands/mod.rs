use crate::repl::Tab;

use super::Repl;

mod env;
mod help;
mod quit;

#[derive(Clone)]
pub struct SlashCommand {
    pub kind: SlashCommandKind,
    pub arguments: Vec<String>,
}

impl std::fmt::Display for SlashCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{} {}", self.kind, self.arguments.join(" "))
    }
}

#[derive(Clone)]
pub enum SlashCommandKind {
    _Empty,
    Help,
    Env,
    Quit,
}

impl std::fmt::Display for SlashCommandKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            SlashCommandKind::_Empty => unreachable!(),
            SlashCommandKind::Help => "help",
            SlashCommandKind::Env => "env",
            SlashCommandKind::Quit => "quit",
        })
    }
}

#[derive(Debug, Clone)]
pub enum SlashCommandBuildError {
    RecievedEmptyString,
    CommandNotFound,
}
impl std::error::Error for SlashCommandBuildError {}
impl std::fmt::Display for SlashCommandBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use SlashCommandBuildError as E;
        f.write_str(match self {
            E::RecievedEmptyString => "Recieved empty string",
            E::CommandNotFound => "Command not found. Try \"/help\"",
        })
    }
}

impl SlashCommand {
    pub const fn empty() -> SlashCommand {
        SlashCommand {
            kind: SlashCommandKind::_Empty,
            arguments: Vec::new(),
        }
    }

    fn from_raw_args<I, C: ToString>(mut args: I) -> Result<SlashCommand, SlashCommandBuildError>
    where
        I: Iterator<Item = C>,
    {
        let cmd = args.next();
        if cmd.is_none() {
            return Err(SlashCommandBuildError::RecievedEmptyString);
        }
        let kind = match SlashCommand::match_cmd_string(cmd.unwrap().to_string()) {
            Some(c) => c,
            None => return Err(SlashCommandBuildError::CommandNotFound),
        };

        let arguments = SlashCommand::build_command_arguments(args);

        Ok(SlashCommand { arguments, kind })
    }

    fn match_cmd_string<S: AsRef<str>>(cmd_str: S) -> Option<SlashCommandKind> {
        Some(match cmd_str.as_ref() {
            "help" => SlashCommandKind::Help,
            "env" => SlashCommandKind::Env,
            "quit" => SlashCommandKind::Quit,
            _ => return None,
        })
    }

    fn build_command_arguments<I, C: ToString>(args: I) -> Vec<String>
    where
        I: Iterator<Item = C>,
    {
        let mut vec = vec![];
        for arg in args {
            let arg = arg.to_string();
            vec.push(arg);
        }

        vec
    }

    pub(crate) fn content(&self) -> &str {
        match self.kind {
            SlashCommandKind::Help => help::content(&self.arguments),
            SlashCommandKind::Env => env::content(&self.arguments),
            SlashCommandKind::Quit => quit::content(),
            SlashCommandKind::_Empty => "",
        }
    }
}

impl Repl {
    pub(super) fn slash_command(&mut self) {
        let command = self.parse_slash_command();
        let previous_tab = self.selected_tab;
        self.selected_tab = Tab::CommandOutput;
        self.slash_command_result = command.clone();

        let cmd = match command {
            Ok(c) => c,
            Err(_) => {
                // Don't execute any commands and print errors to the screen
                return;
            }
        };

        self.execute_slash_command(cmd);
        self.input.buf.clear();
        self.input.height = 0;
        self.input.cursor_index = 0;
        self.selected_tab = previous_tab;
    }

    pub fn parse_slash_command(&self) -> Result<SlashCommand, SlashCommandBuildError> {
        let mut src = self.input.buf.clone();
        src.remove(0); // remove the "/"
        let args = src.split(" ");

        SlashCommand::from_raw_args(args)
    }

    fn execute_slash_command(&mut self, cmd: SlashCommand) {
        match cmd.kind {
            SlashCommandKind::Help => {}
            SlashCommandKind::Env => env::run(self, &cmd.arguments),
            SlashCommandKind::Quit => quit::run(self, &cmd.arguments),
            SlashCommandKind::_Empty => unreachable!(),
        }
    }
}
