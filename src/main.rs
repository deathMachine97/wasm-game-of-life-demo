use rand::Rng;
use std::cmp::Ordering;
use std::io;
use typename::TypeName;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}

pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

impl Universe {
	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}

	fn tick(&mut self) {
		let mut next = self.cells.clone();
		for row in 0..self.height {
			for col in 0..self.width {
				let neighbors_count = self.live_neighbor_count(row, col);
				let cell_index = self.get_index(row, col);
				let cell = self.cells[cell_index];
				let refreshed_cell = match (cell, neighbors_count) {
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					(Cell::Dead, 3) => Cell::Alive,
					(otherwise, _) => otherwise,
				};
				next[cell_index] = refreshed_cell;
			}
		}
		self.cells = next;
	}

	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count: u8 = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}
				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);
				count += self.cells[idx] as u8;
			}
		}
		count
	}
}

use std::fmt;

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				// let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
				let symbol = if cell == Cell::Dead { '-' } else { '+' };
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}
		Ok(())
	}
}

fn main() {
	let mut u1 = Universe {
		height: 4,
		width: 4,
		cells: vec![
			Cell::Dead,		Cell::Alive,		Cell::Dead,		Cell::Dead,
			Cell::Dead,		Cell::Alive,		Cell::Dead,		Cell::Dead,
			Cell::Dead,		Cell::Alive,		Cell::Dead,		Cell::Dead,
			Cell::Dead,		Cell::Dead,			Cell::Dead,		Cell::Dead,
		],
	};
	loop {
		println!("{}",&u1);
		u1.tick();
	}
	println!("\n \n")
	// println!("{}", &u1.live_neighbor_count(1,1));
	// println!("{}", &u1.live_neighbor_count(1,1));

	// let mut count = 0;
	// for col in 1..32 {
	// 	for row in 1..32 {
	// 		count+=1;
	// 	}
	// }
	// println!("{:?}", count);
}
