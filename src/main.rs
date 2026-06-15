use std::io::{self, Write};

use anyhow::Result;
use crossterm::event::Event;
use crossterm::{
    event, execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::Backend;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use resource_collection_sim::{config::SimConfig, map::Map, ui};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    stdout.flush()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run(&mut terminal);
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    while event::poll(std::time::Duration::from_millis(0))? {
        event::read()?;
    }

    let config = SimConfig::default();
    let map = Map::new(config.map_width, config.map_height);
    loop {
        terminal.draw(|frame| ui::draw(frame, &map))?;
        if let Event::Key(_) = event::read()? {
            break;
        }
    }
    Ok(())
}
