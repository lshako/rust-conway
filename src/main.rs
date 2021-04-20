mod cell;
mod grid;
mod opt;

use crate::cell::{Cell, CellColor};
use crate::grid::Grid;
use opt::Opt;
use std::{thread, time};
use structopt::StructOpt;
use termion::{clear, color, cursor, style};

fn main() {
    let opt = Opt::from_args();
    let sleep = 1_000 / opt.fps as u64;

    let mut grid = Grid::new(opt.size, opt.size, opt.concentration, opt.game_type);
    let mut generation = 0;

    loop {
        println!("{}{}", cursor::Goto(1, 1), clear::All);
        println!("{}{:#?}", color::Fg(color::Blue), opt);
        println!(
            "{}generation: {}{}",
            color::Fg(color::Blue),
            color::Fg(color::Yellow),
            generation
        );
        print!("{}", style::Bold);
        for i in 0..grid.width {
            for j in 0..grid.height {
                match grid.get_cell(i, j) {
                    Cell::Alive { color } => {
                        match color {
                            CellColor::C1 => print!("{}", color::Fg(color::Green)),
                            CellColor::C2 => print!("{}", color::Fg(color::Red)),
                        };

                        print!(" o");
                    }
                    Cell::Dead => print!("  "),
                }
            }
            println!("");
        }
        grid = grid.next();
        thread::sleep(time::Duration::from_millis(sleep));
        generation = generation + 1;
    }
}
