use crate::cache::PackageMetadata;
use crate::scanner::Scanner;
use crate::installer::Installer;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};
use ratatui::Frame;
use std::io;
use std::path::PathBuf;

pub struct Tui {
    scanner: Scanner,
    packages: Vec<PackageMetadata>,
    selected_index: usize,
    search_query: String,
    details: Option<PackageMetadata>,
    should_quit: bool,
}

impl Tui {
    pub fn new(scanner: Scanner) -> Self {
        let packages: Vec<PackageMetadata> = scanner.list_all().into_iter().cloned().collect();
        Self {
            scanner,
            packages,
            selected_index: 0,
            search_query: String::new(),
            details: None,
            should_quit: false,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = ratatui::Terminal::new(backend)?;

        loop {
            terminal.draw(|f| self.ui(f))?;

            if self.should_quit {
                break;
            }

            if event::poll(std::time::Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key)?;
                    }
                }
            }
        }

        disable_raw_mode()?;
        execute!(io::stdout(), LeaveAlternateScreen)?;
        Ok(())
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
            .split(f.size());

        let filtered_packages = self.filtered_packages();
        let list_items: Vec<ListItem> = filtered_packages
            .iter()
            .enumerate()
            .map(|(idx, pkg)| {
                let is_selected = idx == self.selected_index;
                let style = if is_selected {
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };

                let name = format!("{} ({})", pkg.name, pkg.package_type);
                ListItem::new(Line::from(vec![Span::styled(name, style)]))
            })
            .collect();

        let mut list_state = ListState::default();
        if !filtered_packages.is_empty() {
            list_state.select(Some(self.selected_index.min(filtered_packages.len() - 1)));
        }

        let list = List::new(list_items)
            .block(Block::default().borders(Borders::ALL).title("Packages"))
            .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

        f.render_stateful_widget(list, chunks[0], &mut list_state);

        let details_text = if let Some(ref pkg) = self.details {
            format!(
                "Name: {}\nVersion: {}\nType: {}\nMaintainer: {}\nDescription: {}\nCategories: {}\nExec: {}\nPath: {}",
                pkg.name,
                pkg.version,
                pkg.package_type,
                pkg.maintainer,
                pkg.description,
                pkg.categories.join(", "),
                pkg.exec_command.as_ref().unwrap_or(&String::new()),
                pkg.source_path.display()
            )
        } else if !filtered_packages.is_empty() {
            let idx = self.selected_index.min(filtered_packages.len() - 1);
            let pkg = &filtered_packages[idx];
            format!(
                "Name: {}\nVersion: {}\nType: {}\nMaintainer: {}\nDescription: {}\nCategories: {}\nExec: {}\nPath: {}",
                pkg.name,
                pkg.version,
                pkg.package_type,
                pkg.maintainer,
                pkg.description,
                pkg.categories.join(", "),
                pkg.exec_command.as_ref().unwrap_or(&String::new()),
                pkg.source_path.display()
            )
        } else {
            "No packages found".to_string()
        };

        let details = Paragraph::new(details_text)
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(Wrap { trim: true });

        f.render_widget(details, chunks[1]);

        let help_text = "↑/↓: Navigate | Enter: Inspect | i: Install | e: Extract | d: Delete | r: Refresh | q: Quit";
        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Help"));

        let help_area = Rect {
            x: 0,
            y: f.size().height.saturating_sub(3),
            width: f.size().width,
            height: 3,
        };
        f.render_widget(help, help_area);
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Char('q') => {
                self.should_quit = true;
            }
            KeyCode::Up => {
                let filtered_len = self.filtered_packages().len();
                if filtered_len > 0 {
                    self.selected_index = self.selected_index.saturating_sub(1);
                    if self.selected_index >= filtered_len {
                        self.selected_index = filtered_len.saturating_sub(1);
                    }
                }
            }
            KeyCode::Down => {
                let filtered = self.filtered_packages();
                if !filtered.is_empty() {
                    self.selected_index = (self.selected_index + 1).min(filtered.len().saturating_sub(1));
                }
            }
            KeyCode::Enter => {
                let filtered = self.filtered_packages();
                if !filtered.is_empty() {
                    let idx = self.selected_index.min(filtered.len() - 1);
                    self.details = Some(filtered[idx].clone());
                }
            }
            KeyCode::Char('i') => {
                let filtered = self.filtered_packages();
                if !filtered.is_empty() {
                    let idx = self.selected_index.min(filtered.len() - 1);
                    let pkg = &filtered[idx];
                    match Installer::install_desktop(pkg, true, false) {
                        Ok(id) => {
                            log::info!("Installed desktop entry: {}", id);
                        }
                        Err(e) => {
                            log::error!("Failed to install: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char('e') => {
                let filtered = self.filtered_packages();
                if !filtered.is_empty() {
                    let idx = self.selected_index.min(filtered.len() - 1);
                    let pkg = &filtered[idx];
                    let out_dir = PathBuf::from("/tmp").join(&pkg.name);
                    match Installer::extract_package(&pkg.source_path, &out_dir) {
                        Ok(_) => {
                            log::info!("Extracted to: {}", out_dir.display());
                        }
                        Err(e) => {
                            log::error!("Failed to extract: {}", e);
                        }
                    }
                }
            }
            KeyCode::Char('d') => {
                let filtered = self.filtered_packages();
                if !filtered.is_empty() {
                    let idx = self.selected_index.min(filtered.len() - 1);
                    let source_path = filtered[idx].source_path.clone();
                    drop(filtered);
                    if let Err(e) = self.scanner.get_cache_mut().remove(&source_path) {
                        log::error!("Failed to remove from cache: {}", e);
                    }
                    self.packages = self.scanner.list_all().into_iter().cloned().collect();
                    if self.selected_index >= self.packages.len() {
                        self.selected_index = self.packages.len().saturating_sub(1);
                    }
                }
            }
            KeyCode::Char('r') => {
                self.packages = self.scanner.list_all().into_iter().cloned().collect();
            }
            _ => {}
        }
        Ok(())
    }

    fn filtered_packages(&self) -> Vec<&PackageMetadata> {
        if self.search_query.is_empty() {
            self.packages.iter().collect()
        } else {
            let query = self.search_query.to_lowercase();
            self.packages
                .iter()
                .filter(|pkg| {
                    pkg.name.to_lowercase().contains(&query) ||
                    pkg.description.to_lowercase().contains(&query) ||
                    pkg.package_type.to_lowercase().contains(&query)
                })
                .collect()
        }
    }
}

