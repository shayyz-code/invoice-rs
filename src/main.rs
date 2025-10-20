use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

mod app;
mod models;
mod pdf;
mod ui;
mod utils;

use app::{App, Mode};
use ui::splash::SplashScreen;

fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::new();
    let mut splash = SplashScreen::new();

    run_app(&mut terminal, &mut app, &mut splash)?;

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    splash: &mut SplashScreen,
) -> anyhow::Result<()> {
    loop {
        terminal.draw(|frame| {
            if !splash.is_done() {
                splash.draw(frame);
            } else {
                ui::layout::draw(frame, app);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    Mode::Normal => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down => app.next(),
                        KeyCode::Up => app.previous(),
                        KeyCode::Char('n') => app.start_new(),
                        KeyCode::Char('e') => app.start_edit(),
                        KeyCode::Char('p') => app.export_pdf(),
                        _ => {}
                    },
                    Mode::Editing => {
                        let form = app.form.as_mut().unwrap();
                        match key.code {
                            KeyCode::Esc => app.cancel_form(),
                            KeyCode::Enter => app.open_save_modal(),
                            KeyCode::Tab => form.next_field(),
                            KeyCode::Backspace => form.backspace(),
                            KeyCode::Char(c) => form.update_field(c),
                            _ => {}
                        }
                    }
                    Mode::Modal(modal_type) => match modal_type {
                        ui::modal::ModalType::Alert => match key.code {
                            _ => app.close_modals(),
                        },
                        ui::modal::ModalType::Confirm => match key.code {
                            KeyCode::Tab => {
                                if let Some(modal) = app.modal.as_mut() {
                                    modal.next_field();
                                }
                            }
                            KeyCode::Esc => app.close_modals(),
                            KeyCode::Enter => {
                                if let Some(mut modal) = app.modal.take() {
                                    modal.press_field(
                                        Box::new(|app: &mut App| {
                                            app.close_modals();
                                        }),
                                        app,
                                    );
                                    app.modal = Some(modal);
                                }
                            }
                            _ => {}
                        },
                    },
                }
            }
        }
    }
    Ok(())
}
