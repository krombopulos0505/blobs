mod balance;
mod blob;
mod grid;
mod position;
mod renderer;
mod simulation;
mod world;

use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use crossterm::{cursor, execute};

use blob::Blob;
use renderer::Renderer;
use simulation::Simulation;

fn main() -> std::io::Result<()> {
    let mut renderer = Renderer::new();
    let mut sim = Simulation::new();
    let seed_blob = Blob::minimal_viable(&mut sim.world, &mut sim.rng);
    sim.blobs.push(seed_blob);

    enable_raw_mode()?;
    execute!(renderer.out, EnterAlternateScreen, cursor::Hide)?;

    let result = run(&mut sim, &mut renderer);

    execute!(renderer.out, LeaveAlternateScreen, cursor::Show)?;
    disable_raw_mode()?;

    result
}

fn run(sim: &mut Simulation, renderer: &mut Renderer) -> std::io::Result<()> {
    loop {
        sim.step();
        renderer.draw(sim)?;

        // non-blocking check for 'q' to quit cleanly
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
