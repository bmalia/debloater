use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute,
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
    Terminal,
};
use std::io;

pub struct ArchTui {
    items: Vec<String>,
    state: ListState,
}

impl ArchTui {
    pub fn new() -> ArchTui {
        let items = vec![
            "Remove Unnecessary Packages".to_string(),
            "Clean Package Cache".to_string(),
            "Remove Orphaned Packages".to_string(),
            "Optimize Pacman".to_string(),
            "Exit".to_string(),
        ];
        let mut state = ListState::default();
        state.select(Some(0));
        ArchTui { items, state }
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
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(size);

                let items: Vec<ListItem> = self
                    .items
                    .iter()
                    .map(|i| ListItem::new(i.clone()))
                    .collect();
                let list = List::new(items)
                    .block(Block::default().borders(Borders::ALL).title("Select operations to perform"))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol(">> ");

                f.render_stateful_widget(list, chunks[0], &mut self.state);
            })?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        break;
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
