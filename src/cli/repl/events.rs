use std::io;

use crossterm::event::{Event, KeyCode, KeyModifiers};

use crate::repl::ReplState;

use super::Repl;

impl Repl {
    pub(super) fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key_e) = crossterm::event::read()? {
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
                            const SCROLL_AMOUNT: u16 = 5;

                            match c {
                                'l' if key_e.modifiers.contains(KeyModifiers::SHIFT) => {
                                    self.scroll_offset.1 += SCROLL_AMOUNT;
                                }
                                'h' if key_e.modifiers.contains(KeyModifiers::SHIFT) => {
                                    self.scroll_offset.1 =
                                        self.scroll_offset.1.saturating_sub(SCROLL_AMOUNT)
                                }
                                'l' => self.selected_tab = self.selected_tab.next(),
                                'h' => self.selected_tab = self.selected_tab.previous(),
                                'j' => self.scroll_offset.0 += SCROLL_AMOUNT,
                                'k' => {
                                    self.scroll_offset.0 =
                                        self.scroll_offset.0.saturating_sub(SCROLL_AMOUNT)
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
                        if self.input.buf.starts_with("/") {
                            self.slash_command();
                        } else if !self.is_input_complete() {
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

    fn is_input_complete(&self) -> bool {
        if let Some(Err(_)) = self.program_result {
            return false;
        }

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
}
