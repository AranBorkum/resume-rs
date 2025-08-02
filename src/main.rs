use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Frame, Terminal,
};

use crate::{
    keymap::global_key_map,
    settings::Settings,
    state::State,
    ui::{
        about_me::render_about_me,
        banner::render_banner,
        contact_details::render_contact_details,
        employment::render_employment,
        keymap::render_keymap,
        loading::render_loading_screen,
        tabs::{render_tabs, TabsHeadings},
    },
};

mod assets;
mod employment;
mod events;
mod keymap;
mod settings;
mod state;
mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal).await;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

async fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = State::default();
    let settings = Settings::default();
    // let _ = state.load_employment_from_file();
    // let _ = state._load_education_from_file(&settings);
    let _ = state.load_employment_file_from_s3(&settings).await;
    let _ = state.load_education_file_from_s3(&settings).await;
    let settings = Settings::default();
    loop {
        terminal.draw(|f| match state.is_loading {
            true => {
                draw_loading_screen(f, &mut state);
            }
            false => {
                draw_app(f, &state);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(settings.poll_duration_ms))? {
            if let event::Event::Key(key) = event::read()? {
                if let event::KeyCode::Char('q') = key.code {
                    break;
                }
                global_key_map(key, &mut state);
            }
        }
    }

    Ok(())
}

fn draw_loading_screen<B: tui::backend::Backend>(f: &mut Frame<B>, state: &mut State) {
    state.update_dot_count();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Percentage(49),
            Constraint::Min(3),
            Constraint::Percentage(49),
        ])
        .split(f.size());
    render_loading_screen(f, chunks[1], &state);
}

fn draw_app<B: tui::backend::Backend>(f: &mut Frame<B>, state: &State) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    render_banner(f, chunks[0]);
    render_tabs(f, chunks[1], state.selected_tab.index());

    match state.selected_tab {
        TabsHeadings::AboutMe => render_about_me(f, chunks[2], &state),
        TabsHeadings::ContactDetails => render_contact_details(f, chunks[2]),
        TabsHeadings::EmploymentAndEducation => render_employment(f, chunks[2], &state),
    }

    render_keymap(f, chunks[3], &state);
}
