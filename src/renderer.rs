use std::io::{Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};

use crate::position::Pos;
use crate::simulation::Simulation;

pub struct Renderer {
    pub out: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Self { out: std::io::stdout() }
    }

    pub fn draw(&mut self, sim: &Simulation) -> std::io::Result<()> {
        for y in 0..sim.world.height() {
            queue!(self.out, MoveTo(0, y as u16))?;
            for x in 0..sim.world.width() {
                let pos = Pos::new(x, y);

                if sim.world.occupant.get(pos).is_some() {
                    queue!(self.out, SetForegroundColor(Color::Yellow), Print("o"))?;
                } else if *sim.world.food.get(pos) {
                    queue!(self.out, SetForegroundColor(Color::Green), Print("%"))?;
                } else if *sim.world.wall.get(pos) {
                    queue!(self.out, SetForegroundColor(Color::White), Print("#"))?;
                } else {
                    queue!(self.out, ResetColor, Print(" "))?;
                }
                queue!(self.out, ResetColor)?;
            }
        }

        self.out.flush()
    }
}
