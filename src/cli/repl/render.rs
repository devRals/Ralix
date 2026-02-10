use ratatui::{
    Frame,
    layout::{Constraint, HorizontalAlignment, Layout, Position, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Tabs},
};

use super::{Repl, Tab};
use crate::{EvalResult, Object, repl::ReplState};

pub(super) const PROMPT: &str = ">>>";
pub(super) const CONTINUATION_PROMPT: &str = "...";
const _HELP_PROMPT: &str = "help>";

impl Repl {
    pub(super) fn draw(&self, f: &mut Frame) {
        let full_area = f.area();
        let r#box = Block::bordered().title(Line::from(" Ralix ".light_magenta()).centered());
        let box_inner = r#box.inner(full_area);

        // let [left, right] = Layout::horizontal([
        //     Constraint::Fill(1),
        //     Constraint::Length(if self.right_side_open { 30 } else { 0 }),
        // ])
        // .areas(box_inner);

        let [tabs_area, screen_area, input_area] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3 + self.input.height),
        ])
        .areas(box_inner);

        f.render_widget(r#box, full_area);
        // self.render_env(f, right);
        self.render_tabs(f, tabs_area);
        self.render_input(f, input_area);
        self.render_screen(f, screen_area);

        if self.show_help {
            self.render_help(f);
        }
    }

    fn render_help(&self, f: &mut Frame) {
        let area = f.area();
        let block = Block::bordered()
            .title("Help")
            .style(Style::new().fg(Color::LightBlue));
        let help_text = r#"Key bindings:

    - <C-?>: Toggle this help popup
    - Ctrl + c: Stop execution
    - Ctrl + l: Go to next tab
    - Ctrl + h: Go to previous tab
    - Ctrl + e: Toggle environment panel
    - Ctrl + j: Scroll down
    - Ctrl - k: Scroll up
    - Esc: Quit or "/quit""#;

        let paragraph = Paragraph::new(help_text).white().block(block);

        // Create a new area in the center of the screen
        let area = centered_rect(60, 50, area);
        f.render_widget(Clear, area); //this clears the background
        f.render_widget(paragraph, area);
    }

    fn get_cursor_position(&self) -> (u16, u16) {
        let mut row = 0;
        let mut col = 0;
        for (i, char) in self.input.buf.chars().enumerate() {
            if i == self.input.cursor_index {
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

    // fn render_env(&self, f: &mut Frame, area: Rect) {
    //     let mut items: Vec<_> = self.context.env.current_items().into_iter().collect();
    //     items.sort_by_key(|(ident, _)| ident.to_string());
    //
    //     let rows: Vec<_> = items
    //         .into_iter()
    //         .map(|(ident, value)| {
    //             Row::new(vec![
    //                 Cell::from(ident.to_string()),
    //                 Cell::from(value.to_string()),
    //             ])
    //         })
    //         .collect();
    //     let widths = [Constraint::Length(10), Constraint::Length(20)];
    //     let table = Table::new(rows, widths).block(
    //         Block::new()
    //             .borders(Borders::LEFT)
    //             .title("Environment".light_cyan()),
    //     );
    //     f.render_widget(table, area);
    // }

    fn render_screen(&self, f: &mut Frame, area: Rect) {
        match self.selected_tab {
            Tab::Evaluation => self.draw_evaluation(f, area),
            Tab::Ast => self.draw_ast(f, area),
            Tab::Errors => self.draw_errors(f, area),
            Tab::CommandOutput => self.draw_command_output(f, area),
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
                EvalResult::Value(v) => colorize_obj(v),
                EvalResult::Return(v) => match v {
                    Some(v) => colorize_obj(v),
                    None => "No Value".dark_gray(),
                },
                EvalResult::NoValue => "No Value".dark_gray(),
                EvalResult::Err(err) => err.to_string().red(),
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

    fn build_input_string(&self) -> String {
        if let ReplState::Running = self.state {
            return "The program is running please wait".dark_gray().to_string();
        }

        let prompt_color = if let Some(r) = &self.program_result {
            match r {
                Ok(_) => Stylize::green,
                Err(_) => Stylize::light_red,
            }
        } else {
            Stylize::light_magenta
        };

        if self.input.buf.is_empty() {
            return format!("{} ", prompt_color(PROMPT));
        }

        let mut styled_lines = String::new();
        let lines: Vec<&str> = self.input.buf.split('\n').collect();

        styled_lines.push_str(&format!("{} {}", prompt_color(PROMPT), lines[0]));
        for line in &lines[1..] {
            styled_lines.push_str(&format!("\n{} {}", CONTINUATION_PROMPT.dark_gray(), line));
        }

        styled_lines
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
            .white()
            .select(Some(self.selected_tab as usize));

        f.render_widget(tabs, area);
    }

    fn draw_command_output(&self, f: &mut Frame, area: Rect) {
        match &self.slash_command_result {
            Ok(cmd) => {
                let content = Paragraph::new(cmd.content()).scroll(self.scroll_offset);
                f.render_widget(content, area);
            }
            Err(err) => f.render_widget(err.to_string().red(), area),
        }
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

fn colorize_obj(obj: &Object) -> Span<'_> {
    match obj {
        Object::Int(v) => v.light_yellow(),
        Object::Float(v) => v.light_yellow(),
        Object::Char(v) => format!("'{v}'").light_cyan(),
        Object::String(v) => format!("\"{v}\"").light_green(),
        Object::Boolean(v) => v.cyan(),
        Object::Type(v) => v.to_string().light_yellow(),
        Object::Address(v) => format!("<{v:?}>").dark_gray(),
        Object::Null => "null".dark_gray(),
        Object::Function(func) => func.to_string().white(),
        Object::Array(_) => obj.to_string().white(),
        Object::HashMap(_) => obj.to_string().white(),
    }
}
