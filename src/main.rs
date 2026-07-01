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

use resource_collection_sim::{config::SimConfig, simulation::Simulation, ui};
use std::time::{Duration, Instant};

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
    while event::poll(Duration::ZERO)? {
        event::read()?;
    }

    let mut sim = Simulation::new(SimConfig::default());
    let tick_rate = sim.config.tick_rate;
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|frame| ui::draw(frame, &sim))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);
        if event::poll(timeout)? {
            if let Event::Key(_) = event::read()? {
                break;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sim.update();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
