use std::{io, time::Duration};

use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, HorizontalAlignment, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs},
};

use crate::{
    Environment, EvalResult, Evaluator, ExecuteError, Object, Program, SymbolTable,
    parse_with_symbol_table,
};

const PROMPT: &str = ">>>";
const CONTINUATION_PROMPT: &str = "...";
const _HELP_PROMPT: &str = "help>";

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum Tab {
    Evaluation,
    Ast,
    Errors,
}

impl Tab {
    const ALL_TABS: [Tab; 3] = [Tab::Evaluation, Tab::Ast, Tab::Errors];
    const ALL_TABS_STR: [&str; 3] = ["Evaluation", "Abstract Syntax Tree", "Errors"];

    fn next(self) -> Self {
        let i = Self::ALL_TABS.iter().position(|&t| t == self).unwrap();
        Self::ALL_TABS[(i + 1) % Self::ALL_TABS.len()]
    }

    fn previous(self) -> Self {
        let i = Self::ALL_TABS.iter().position(|&t| t == self).unwrap();
        Self::ALL_TABS[(i.saturating_sub(1)) % Self::ALL_TABS.len()]
    }
}

struct ReplContext {
    env: Environment,
    symbol_table: SymbolTable,
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
    refresh_rate: u64,
    program_result: Option<Result<Program, ExecuteError>>,
    eval_result: EvalResult<Object>,
    scroll_offset: (u16, u16),
    show_help: bool,
    right_side_open: bool,

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
            refresh_rate: 400,
            program_result: None,
            eval_result: EvalResult::NoValue,
            scroll_offset: (0, 0),
            show_help: false,
            right_side_open: true,

            context: ReplContext {
                symbol_table: SymbolTable::default(),
                env: Environment::default(),
            },
        }
    }

    pub fn run(&mut self, term: &mut DefaultTerminal) -> color_eyre::Result<()> {
        while !self.should_quit {
            term.draw(|f| self.draw(f))?;
            self.handle_events()?;
            self.parse();
        }

        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let full_area = f.area();
        let r#box = Block::bordered().title(Line::from(" Ralix ".light_magenta()).centered());
        let box_inner = r#box.inner(full_area);

        let [left, right] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(if self.right_side_open { 30 } else { 0 }),
        ])
        .areas(box_inner);

        let [tabs_area, screen_area, input_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3 + self.input.height),
        ])
        .areas(left);

        f.render_widget(r#box, full_area);
        self.render_env(f, right);
        self.render_tabs(f, tabs_area);
        self.render_input(f, input_area);
        self.render_screen(f, screen_area);

        if self.show_help {
            self.render_help(f);
        }
    }

    fn render_help(&self, f: &mut Frame) {
        let area = f.area();
        let block = Block::default().title("Help").borders(Borders::ALL);
        let help_text = "Key bindings:\n\n       - <C-?>: Toggle this help popup\n       - Ctrl + c: Stop execution\n       - Ctrl + l: Go to next tab\n       - Ctrl + h: Go to previous tab\n         - Ctrl + e: Toggle environment panel\n       - Esc: Quit";
        let paragraph = Paragraph::new(help_text).block(block);

        // Create a new area in the center of the screen
        let area = centered_rect(60, 50, area);
        f.render_widget(Clear, area); //this clears the background
        f.render_widget(paragraph, area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if crossterm::event::poll(Duration::from_millis(self.refresh_rate))?
            && let Event::Key(key_e) = crossterm::event::read()?
        {
            match self.state {
                ReplState::Editing => match key_e.code {
                    KeyCode::Esc => {
                        if self.show_help {
                            self.show_help = false;
                        } else {
                            self.should_quit = true
                        }
                    }
                    KeyCode::Left => {
                        self.input.cursor_index = self.input.cursor_index.saturating_sub(1);
                    }
                    KeyCode::Right => {
                        let char_count = self.input.buf.chars().count();
                        self.input.cursor_index = self.input.cursor_index.saturating_add(1);
                        if self.input.cursor_index > char_count {
                            self.input.cursor_index = char_count;
                        }
                    }
                    KeyCode::Char(c) => {
                        if c == '?' && self.input.buf.is_empty() {
                            self.show_help = !self.show_help;
                            return Ok(());
                        }

                        if key_e.modifiers.contains(KeyModifiers::CONTROL) {
                            match c {
                                'l' if key_e.modifiers.contains(KeyModifiers::SHIFT) => {
                                    self.scroll_offset.1 += 1
                                }
                                'h' if key_e.modifiers.contains(KeyModifiers::SHIFT) => {
                                    self.scroll_offset.1 = self.scroll_offset.1.saturating_sub(1)
                                }
                                'l' => self.selected_tab = self.selected_tab.next(),
                                'h' => self.selected_tab = self.selected_tab.previous(),
                                'j' => self.scroll_offset.0 += 1,
                                'k' => {
                                    self.scroll_offset.0 = self.scroll_offset.0.saturating_sub(1)
                                }
                                'e' => self.right_side_open = !self.right_side_open,

                                _ => {}
                            }
                            return Ok(());
                        }

                        let byte_idx = self
                            .input
                            .buf
                            .char_indices()
                            .nth(self.input.cursor_index)
                            .map(|(i, _)| i)
                            .unwrap_or(self.input.buf.len());
                        self.input.buf.insert(byte_idx, c);
                        self.input.cursor_index = self.input.cursor_index.saturating_add(1);
                    }
                    KeyCode::Backspace => {
                        if self.input.cursor_index > 0 {
                            let new_cursor_pos = self.input.cursor_index - 1;
                            let byte_idx = self
                                .input
                                .buf
                                .char_indices()
                                .nth(new_cursor_pos)
                                .map(|(i, _)| i)
                                .unwrap(); // Safe due to cursor_index > 0

                            let removed_char = self.input.buf.remove(byte_idx);

                            if removed_char == '\n' {
                                self.input.height = self.input.height.saturating_sub(1);
                            }
                            self.input.cursor_index = new_cursor_pos;
                        }
                    }
                    KeyCode::Enter => {
                        if !self.is_input_complete() {
                            self.input.buf.insert(self.input.cursor_index, '\n');
                            self.input.height += 1;
                            self.input.cursor_index = self.input.cursor_index.saturating_add(1);
                        } else {
                            self.state = ReplState::Running;
                            self.eval_result = self.execute();
                        }
                    }
                    _ => {}
                },

                ReplState::Running => match key_e.code {
                    KeyCode::Char('c') if key_e.modifiers.contains(KeyModifiers::CONTROL) => {
                        self.state = ReplState::Editing
                    }
                    _ => {}
                },
            }
        }

        Ok(())
    }

    fn is_cursor_at_the_end(&self) -> bool {
        self.input.cursor_index == self.input.buf.chars().count()
    }

    fn is_input_complete(&self) -> bool {
        let mut stack = Vec::new();
        for char in self.input.buf.chars() {
            match char {
                '(' | '{' | '[' => stack.push(char),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                _ => (),
            }
        }
        stack.is_empty()
    }

    fn get_cursor_position(&self) -> (u16, u16) {
        let mut row = 0;
        let mut col = 0;
        for (i, char) in self.input.buf.chars().enumerate() {
            if i == self.input.cursor_index as usize {
                break;
            }
            if char == '\n' {
                row += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        (row, col)
    }

    fn build_input_string(&self) -> String {
        if let ReplState::Running = self.state {
            return "The program is running please wait".dark_gray().to_string();
        }
        let mut styled_lines = String::new();

        if self.input.buf.is_empty() {
            return format!("{} ", PROMPT.light_magenta());
        }

        let lines: Vec<&str> = self.input.buf.split('\n').collect();

        styled_lines.push_str(&format!("{} {}", PROMPT.light_magenta(), lines[0]));
        for line in &lines[1..] {
            styled_lines.push_str(&format!("\n{} {}", CONTINUATION_PROMPT.dark_gray(), line));
        }

        styled_lines
    }

    fn render_env(&self, f: &mut Frame, area: Rect) {
        let mut items: Vec<_> = self.context.env.current_items().into_iter().collect();
        items.sort_by_key(|(ident, _)| ident.to_string());

        let rows: Vec<_> = items
            .into_iter()
            .map(|(ident, value)| {
                Row::new(vec![
                    Cell::from(ident.to_string()),
                    Cell::from(value.to_string()),
                ])
            })
            .collect();
        let widths = [Constraint::Length(10), Constraint::Length(20)];
        let table = Table::new(rows, widths)
            .block(Block::new().borders(Borders::LEFT).title("Environment"));
        f.render_widget(table, area);
    }

    fn render_screen(&self, f: &mut Frame, area: Rect) {
        match self.selected_tab {
            Tab::Evaluation => self.draw_evaluation(f, area),
            Tab::Ast => self.draw_ast(f, area),
            Tab::Errors => self.draw_errors(f, area),
        }
    }

    fn draw_evaluation(&self, f: &mut Frame, area: Rect) {
        if let Some(result) = &self.program_result {
            match result {
                Ok(_) => {}
                Err(_) => {
                    f.render_widget(
                        "Your input has some errors go check them out in the `Errors` section"
                            .red(),
                        area,
                    );
                    return;
                }
            }
        }

        f.render_widget(
            match &self.eval_result {
                EvalResult::Value(v) => v.to_string(),
                EvalResult::Return(v) => match v {
                    Some(v) => v.to_string(),
                    None => "No Value".dark_gray().to_string(),
                },
                EvalResult::NoValue => "No Value".dark_gray().to_string(),
                EvalResult::Err(err) => err.to_string(),
            },
            area,
        );
    }

    fn draw_ast(&self, f: &mut Frame, area: Rect) {
        match &self.program_result {
            Some(program_result) => match program_result {
                Ok(program) => {
                    let program_as_json = serde_json::to_string_pretty(program).unwrap();
                    let text = Paragraph::new(program_as_json).scroll(self.scroll_offset);
                    f.render_widget(text, area);
                }
                Err(_) => f.render_widget(
                    "Your input has some errors go check them out in the `Errors` section".red(),
                    area,
                ),
            },

            None => f.render_widget("Please fill up your input".dark_gray(), area),
        }
    }

    fn draw_errors(&self, f: &mut Frame, area: Rect) {
        match &self.program_result {
            Some(program_result) => match program_result {
                Ok(_) => f.render_widget("Your program has no errors! Yippie!".green(), area),
                Err(err) => f.render_widget(Paragraph::new(err.to_string()).red(), area),
            },
            None => f.render_widget("Please fill up your input".dark_gray(), area),
        }
    }

    fn render_input(&self, f: &mut Frame, area: Rect) {
        let msg = self.build_input_string();
        let input_text = Paragraph::new(msg).block(Block::new().borders(Borders::TOP));

        if let ReplState::Editing = self.state {
            let (row, col) = self.get_cursor_position();
            let prompt_offset = 4;
            f.set_cursor_position(Position::new(
                area.x + col + prompt_offset,
                area.y + row + 1,
            ));
        } else {
            f.set_cursor_position(Position::new(0, 0));
        }

        f.render_widget(input_text, area);
    }

    fn render_tabs(&self, f: &mut Frame, area: Rect) {
        let tabs = Tab::ALL_TABS_STR;

        let active_style = Style::new()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
            .add_modifier(Modifier::ITALIC);
        let tabs = Tabs::new(tabs)
            .highlight_style(active_style)
            .block(
                Block::new()
                    .borders(Borders::from_bits_retain(
                        Borders::TOP.bits() | Borders::BOTTOM.bits(),
                    ))
                    .title_top("\"?\" for help")
                    .title_alignment(HorizontalAlignment::Right),
            )
            .select(Some(self.selected_tab as usize));

        f.render_widget(tabs, area);
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
            let mut evaluator = Evaluator::new(&mut self.context.env);
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

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::vertical([
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

    Layout::horizontal([
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
