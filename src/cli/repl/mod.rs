use ratatui::DefaultTerminal;

use crate::{
    Environment, EvalResult, Evaluator, ExecuteError, Heap, Object, Program, SymbolTable,
    parse_with_symbol_table,
};

use commands::{SlashCommand, SlashCommandBuildError};

mod commands;
mod events;
mod render;

pub mod legacy;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Tab {
    Evaluation,
    Ast,
    Errors,

    CommandOutput,
}

impl Tab {
    const ALL_TABS: [Tab; 3] = [Tab::Evaluation, Tab::Ast, Tab::Errors];
    const ALL_TABS_STR: [&str; 3] = ["Evaluation", "Abstract Syntax Tree", "Errors"];

    fn next(self) -> Self {
        if !Tab::ALL_TABS.contains(&self) {
            return Self::ALL_TABS[0];
        }

        let i = Self::ALL_TABS.iter().position(|&t| t == self).unwrap();
        Self::ALL_TABS[(i + 1) % Self::ALL_TABS.len()]
    }

    fn previous(self) -> Self {
        if !Tab::ALL_TABS.contains(&self) {
            return Self::ALL_TABS[0];
        }

        let i = Self::ALL_TABS.iter().position(|&t| t == self).unwrap();
        Self::ALL_TABS[(i.saturating_sub(1)) % Self::ALL_TABS.len()]
    }
}

struct ReplContext {
    env: Environment,
    symbol_table: SymbolTable,
    heap: Heap,
}

enum ReplState {
    Editing,
    Running,
}

struct Input {
    buf: String,
    height: u16,
    cursor_index: usize,
}

pub struct Repl {
    should_quit: bool,
    input: Input,
    selected_tab: Tab,
    state: ReplState,
    program_result: Option<Result<Program, ExecuteError>>,
    eval_result: EvalResult<Object>,
    scroll_offset: (u16, u16),
    show_help: bool,
    right_side_open: bool,

    slash_command_result: Result<SlashCommand, SlashCommandBuildError>,

    context: ReplContext,
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

impl Repl {
    pub fn new() -> Repl {
        Repl {
            should_quit: false,
            input: Input {
                buf: String::new(),
                height: 0,
                cursor_index: 0,
            },
            selected_tab: Tab::Evaluation,
            state: ReplState::Editing,
            program_result: None,
            eval_result: EvalResult::NoValue,
            scroll_offset: (0, 0),
            show_help: false,
            right_side_open: true,

            context: ReplContext {
                symbol_table: SymbolTable::default(),
                env: Environment::default(),
                heap: Heap::new(),
            },

            slash_command_result: Ok(SlashCommand::empty()),
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_quit {
            term.draw(|f| self.draw(f))?;
            self.handle_events()?;

            if !self.input.buf.starts_with("/") {
                self.parse();
            } else {
                self.slash_command_result = self.parse_slash_command();
                self.selected_tab = Tab::CommandOutput;
            }
        }

        Ok(())
    }

    fn parse(&mut self) {
        if self.input.buf.is_empty() {
            self.program_result = None
        } else {
            self.program_result = Some(parse_with_symbol_table(
                &self.input.buf,
                &mut self.context.symbol_table,
            ));
        }
    }

    fn execute(&mut self) -> EvalResult<Object> {
        if self.input.buf.is_empty() {
            return EvalResult::NoValue;
        }

        self.program_result = Some(parse_with_symbol_table(
            &self.input.buf,
            &mut self.context.symbol_table,
        ));

        self.state = ReplState::Running;
        let res = if let Some(Ok(program)) = &self.program_result {
            let mut evaluator = Evaluator::new(&mut self.context.env, &mut self.context.heap);
            evaluator.evaluate_program(program.clone())
        } else {
            EvalResult::NoValue
        };

        self.state = ReplState::Editing;
        self.input.buf.clear();
        self.input.height = 0;
        self.input.cursor_index = 0;

        res
    }
}
