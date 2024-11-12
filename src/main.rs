use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use std::io;

#[derive(PartialEq)]
enum Page {
    Welcome,
    Home,
}

struct App {
    page: Page,
}

impl App {
    fn new() -> App {
        App {
            page: Page::Welcome,
        }
    }
}

fn main() -> Result<(), io::Error> {
    // setting up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // main loop

    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            // basic layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(0),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            // render different content based on the current page
            let content = match app.page {
                Page::Welcome => "solana at your fingertips\n\nPress h to go home",
                Page::Home => "welcome home\nyour wallets are here",
            };

            // header
            f.render_widget(
                Block::default()
                    .title("terminal wallet")
                    .borders(Borders::ALL),
                chunks[0],
            );

            // main content
            f.render_widget(
                Paragraph::new(content)
                    .block(Block::default().borders(Borders::ALL))
                    .style(Style::default().fg(Color::White)),
                chunks[1],
            );

            // footer
            f.render_widget(
                Paragraph::new("press q to quit").block(Block::default().borders(Borders::ALL)),
                chunks[2],
            );
        })?;

        // handle input events
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('h') => app.page = Page::Home,
                    KeyCode::Char('w') => app.page = Page::Welcome,
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
