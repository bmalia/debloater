use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    text::{Span, Spans},
    Terminal,
};
use std::io;

#[derive(Clone)]
struct MenuItem {
    name: String,
    is_category: bool,
    selected: bool,
    indent_level: usize,
}

pub struct ArchTui {
    items: Vec<MenuItem>,
    state: ListState,
}

impl ArchTui {
    pub fn new() -> ArchTui {
        let items = vec![
            MenuItem { name: "Package management".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Clean cache".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Remove orphaned packages".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual unused package removal".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Flatpack management".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Repair libraries".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Remove unused libraries".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual unused flatpak removal".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Change installation directory".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Cache and logs".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Clear systemd journal".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Clean general logs".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Clean user cache".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Config".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "pac* file management".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual orphaned config removal".to_string(), is_category: false, selected: false, indent_level: 1 },
        ];
        
        let mut state = ListState::default();
        state.select(Some(0));
        ArchTui { items, state }
    }

    fn toggle_category(&mut self, category_index: usize) {
        if !self.items[category_index].is_category {
            return;
        }

        // Get the state to set (opposite of current category state)
        let new_state = !self.items[category_index].selected;
        self.items[category_index].selected = new_state;

        // Find the range of items in this category
        let mut end_index = category_index + 1;
        while end_index < self.items.len() && !self.items[end_index].is_category {
            self.items[end_index].selected = new_state;
            end_index += 1;
        }
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, Clear(ClearType::All))?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(3),
                        Constraint::Length(3),
                    ].as_ref())
                    .split(size);

                let items: Vec<ListItem> = self.items
                    .iter()
                    .map(|item| {
                        let indent = "  ".repeat(item.indent_level);
                        let prefix = if item.is_category {
                            format!("{}{} ", indent, if item.selected { "[x]" } else { "[ ]" })
                        } else {
                            format!("{}{} ", indent, if item.selected { "[x]" } else { "[ ]" })
                        };
                        let style = if item.is_category {
                            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default()
                        };
                        ListItem::new(format!("{}{}", prefix, item.name)).style(style)
                    })
                    .collect();

                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Select operations to perform with the arrow keys and enter"))
                    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[0], &mut self.state);

                // Bottom buttons
                let button_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ].as_ref())
                    .split(chunks[1]);

                let confirm_text = Spans::from(vec![
                    Span::styled("Press ", Style::default()),
                    Span::styled("c", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(" to confirm", Style::default()),
                ]);
                let exit_text = Spans::from(vec![
                    Span::styled("Press ", Style::default()),
                    Span::styled("q", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled(" to exit", Style::default()),
                ]);

                let confirm_block = Paragraph::new(confirm_text)
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(tui::layout::Alignment::Center);
                let exit_block = Paragraph::new(exit_text)
                    .block(Block::default().borders(Borders::ALL))
                    .alignment(tui::layout::Alignment::Center);

                f.render_widget(confirm_block, button_layout[0]);
                f.render_widget(exit_block, button_layout[1]);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        break;
                    }
                    KeyCode::Char('c') => {
                        disable_raw_mode()?;
                        execute!(io::stdout(), Clear(ClearType::All))?;
                        println!("Selected operations:");
                        for item in &self.items {
                            if !item.is_category && item.selected {
                                println!("- {}", item.name);
                            }
                        }
                        break;
                    }
                    KeyCode::Enter => {
                        if let Some(i) = self.state.selected() {
                            if self.items[i].is_category {
                                self.toggle_category(i);
                            } else {
                                self.items[i].selected = !self.items[i].selected;
                            }
                        }
                    }
                    KeyCode::Down => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i >= self.items.len() - 1 {
                                    0
                                } else {
                                    i + 1
                                }
                            }
                            None => 0,
                        };
                        self.state.select(Some(i));
                    }
                    KeyCode::Up => {
                        let i = match self.state.selected() {
                            Some(i) => {
                                if i == 0 {
                                    self.items.len() - 1
                                } else {
                                    i - 1
                                }
                            }
                            None => 0,
                        };
                        self.state.select(Some(i));
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), io::Error> {
    let mut tui = ArchTui::new();
    tui.run()
}
