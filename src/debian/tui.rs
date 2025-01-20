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
use crate::debian::operations::DebianOperation;
use crate::operation_descriptions::get_description;

#[derive(Clone)]
struct MenuItem {
    name: String,
    is_category: bool,
    selected: bool,
    indent_level: usize,
}

enum Screen {
    Selection,
    Confirmation,
}

pub struct DebianTui {
    items: Vec<MenuItem>,
    state: ListState,
    current_screen: Screen,
}

impl DebianTui {
    pub fn new() -> DebianTui {
        let items = vec![
            MenuItem { name: "Package management".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Clean cache".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Remove orphaned packages".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual unused package removal (coming soon)".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Flatpack management".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Repair libraries".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Remove unused libraries".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual unused flatpak removal (coming soon)".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Change installation directory (coming soon)".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Cache and logs".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "Clear systemd journal".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Clean general logs (deprecated)".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Clean user cache".to_string(), is_category: false, selected: false, indent_level: 1 },
            
            MenuItem { name: "Config".to_string(), is_category: true, selected: false, indent_level: 0 },
            MenuItem { name: "pac* file management".to_string(), is_category: false, selected: false, indent_level: 1 },
            MenuItem { name: "Manual orphaned config removal (coming soon)".to_string(), is_category: false, selected: false, indent_level: 1 },
        ];
        
        let mut state = ListState::default();
        state.select(Some(0));
        ArchTui { 
            items, 
            state, 
            current_screen: Screen::Selection,
        }
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

    fn get_operation_for_item(&self, item_name: &str) -> Option<ArchOperation> {
        match item_name {
            "Clean cache" => Some(ArchOperation::CleanCache),
            "Remove orphaned packages" => Some(ArchOperation::RemoveOrphaned),
            "Manual unused package removal" => Some(ArchOperation::ManualPackageRemoval),
            "Repair libraries" => Some(ArchOperation::RepairFlatpak),
            "Remove unused libraries" => Some(ArchOperation::RemoveUnusedFlatpak),
            "Manual unused flatpak removal" => Some(ArchOperation::ManualFlatpakRemoval),
            "Change installation directory" => Some(ArchOperation::ChangeFlatpakDir),
            "Clear systemd journal" => Some(ArchOperation::ClearSystemdJournal),
            "Clean general logs" => Some(ArchOperation::CleanGeneralLogs),
            "Clean user cache" => Some(ArchOperation::CleanUserCache),
            "pac* file management" => Some(ArchOperation::ManagePacFiles),
            "Manual orphaned config removal" => Some(ArchOperation::RemoveOrphanedConfigs),
            _ => None,
        }
    }

    fn execute_selected_operations(&self) -> Vec<Result<(), String>> {
        let mut results = Vec::new();
        
        println!("\nSelected operations to execute:");
        for item in &self.items {
            if !item.is_category && item.selected {
                println!("• {}", item.name);
                if let Some(operation) = self.get_operation_for_item(&item.name) {
                    results.push(operation.execute());
                }
            }
        }
        
        if results.is_empty() {
            println!("No operations selected!");
        }
        
        results
    }

    fn draw_confirmation_screen<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        area: tui::layout::Rect,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(3),
            ].as_ref())
            .split(area);

        // Title
        let title = Paragraph::new("Confirm Operations")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(tui::layout::Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(title, chunks[0]);

        // Selected operations
        let selected_ops: Vec<ListItem> = self.items.iter()
            .filter(|item| !item.is_category && item.selected)
            .map(|item| ListItem::new(format!("• {}", item.name)))
            .collect();

        let operations_list = List::new(selected_ops)
            .block(Block::default().borders(Borders::ALL).title("Selected Operations"))
            .style(Style::default());
        f.render_widget(operations_list, chunks[1]);

        // Buttons
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(chunks[2]);

        let confirm = Paragraph::new(Spans::from(vec![
            Span::styled("Press ", Style::default()),
            Span::styled("Enter", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::styled(" to execute", Style::default()),
        ]))
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

        let cancel = Paragraph::new(Spans::from(vec![
            Span::styled("Press ", Style::default()),
            Span::styled("Esc", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            Span::styled(" to cancel", Style::default()),
        ]))
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

        f.render_widget(confirm, button_layout[0]);
        f.render_widget(cancel, button_layout[1]);
    }

    fn draw_selection_screen<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        size: tui::layout::Rect,
    ) {
        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(30),
                Constraint::Percentage(70),
            ].as_ref())
            .split(size);

        // Description panel on the left
        let description = if let Some(selected) = self.state.selected() {
            let item = &self.items[selected];
            if !item.is_category {
                let desc = get_description(&item.name);
                vec![
                    Spans::from(Span::styled(desc.title, Style::default().add_modifier(Modifier::BOLD))),
                    Spans::from(""),
                    Spans::from(desc.description),
                ]
            } else {
                vec![Spans::from("Select an operation to see its description")]
            }
        } else {
            vec![Spans::from("Select an operation to see its description")]
        };

        let description_widget = Paragraph::new(description)
            .block(Block::default().borders(Borders::ALL).title("Description"))
            .wrap(tui::widgets::Wrap { trim: true });
        f.render_widget(description_widget, main_chunks[0]);

        // Right side (operations list and buttons)
        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),
                Constraint::Length(3),
            ].as_ref())
            .split(main_chunks[1]);

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
            .block(Block::default().borders(Borders::ALL).title("Select operations to perform"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, right_chunks[0], &mut self.state.clone());

        // Bottom buttons
        let button_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ].as_ref())
            .split(right_chunks[1]);

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
                match self.current_screen {
                    Screen::Selection => self.draw_selection_screen(f, size),
                    Screen::Confirmation => self.draw_confirmation_screen(f, size),
                }
            })?;

            if let Event::Key(key) = event::read()? {
                match self.current_screen {
                    Screen::Selection => {
                        match key.code {
                            KeyCode::Char('q') => {
                                disable_raw_mode()?;
                                break;
                            }
                            KeyCode::Char('c') => {
                                // Check if any operations are selected
                                if self.items.iter().any(|item| !item.is_category && item.selected) {
                                    self.current_screen = Screen::Confirmation;
                                }
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
                    Screen::Confirmation => {
                        match key.code {
                            KeyCode::Enter => {
                                disable_raw_mode()?;
                                execute!(io::stdout(), Clear(ClearType::All))?;
                                let results = self.execute_selected_operations();
                                if !results.is_empty() {
                                    println!("\nExecution results:");
                                    for result in results {
                                        match result {
                                            Ok(_) => (),
                                            Err(e) => eprintln!("Error: {}", e),
                                        }
                                    }
                                }
                                break;
                            }
                            KeyCode::Esc => {
                                self.current_screen = Screen::Selection;
                            }
                            _ => {}
                        }
                    }
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
