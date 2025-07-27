use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    Terminal,
};

use crate::{
    keymap::global_key_map, settings::Settings, state::State, ui::{
        about_me::render_about_me, banner::render_banner, contact_details::render_contact_details, employment::render_employment, keymap::render_keymap, tabs::{render_tabs, TabsHeadings}
    }
};

mod assets;
mod employment;
mod events;
mod keymap;
mod settings;
mod state;
mod ui;


fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

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

fn run_app<B: tui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut state = State::default();
    let settings = Settings::default();
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints([
                    Constraint::Length(9),
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(size);

            render_banner(f, chunks[0]);
            render_tabs(f, chunks[1], state.selected_tab.index());

            match state.selected_tab {
                TabsHeadings::AboutMe => render_about_me(f, chunks[2]),
                TabsHeadings::ContactDetails => render_contact_details(f, chunks[2]),
                TabsHeadings::EmploymentAndEducation => render_employment(f, chunks[2], &state),
            }

            render_keymap(f, chunks[3], &state);
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
