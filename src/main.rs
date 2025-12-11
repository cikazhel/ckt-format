use std::{io::{self, stdout}, thread::sleep, time::Duration};
use rand::Rng;
use crossterm::{self, ExecutableCommand, cursor, terminal::{Clear, ClearType}};

struct Session {
	mines: Box<[u8]>,
	board: Box<[u8]>,
	size_x: u16,
	size_y: u16
}

impl Session {
    fn new(size_x: u16, size_y: u16) -> Self {
        let mut randgen = rand::rng();
        
        let size = size_x*size_y;
        let size_bytes = (size+8-1)/8;
        let mut mines = (vec![0; size_bytes as usize]).into_boxed_slice();
        let board = (vec![0; size as usize]).into_boxed_slice();

        for _ in 0..5 {
            let idx_byte = randgen.random_range(0..mines.len());
            let idx_mask = 1 << randgen.random_range(0..8);
            let state = mines[idx_byte];
            mines[idx_byte] = state | idx_mask;
        }

        return Self {
            mines: mines,
            board: board,
            size_x: size_x,
            size_y: size_y
        };
    }

	fn coordinate_to_idx(&mut self, x: u16, y: u16) -> usize {
		return (x + y * self.size_x) as usize;
	}

	fn is_mine(&mut self, x: u16, y: u16) -> bool {
		let idx = self.coordinate_to_idx(x, y);
		let idx_byte = idx/8;
		let chunk = self.mines[idx_byte as usize];
		return chunk & (1 << (idx % 8)) != 0;
	}
    
	fn render(&mut self) {
		clear().expect("Failed to clear");
		for y in 0..self.size_y {
			print!("[");
			for x in 0..self.size_x {
				let state = self.board[self.coordinate_to_idx(x, y) as usize];
				let display: char = get_char(state);
				if self.is_mine(x, y) == true {
					print!(" + ");
					continue;
				}
				print!(" {display} ");
			}
			print!("]\n");
		}
	}

	fn reveal(&mut self, x: u16, y: u16, manual: bool) {
		if manual && self.is_mine(x, y) {
			println!("sorry twin u lost :(");
			return;
		}

		self.board[self.coordinate_to_idx(x, y)] = 11;

		let mut empty_spaces: Vec<[u16; 2]> = Vec::with_capacity(8);
		let mut mines: u8 = 0;

		for offy in -1..=1 {
			for offx in -1..=1 {
				if offx == 0 && offy == 0 {
					continue;
				}

				let nx = x as i16+offx;
				if nx < 0 {
					continue;
				}

				let ny = y as i16+offy;
				if ny < 0 {
					continue;
				}

				let nx = nx as u16;
				let ny = ny as u16;

				if nx >= self.size_x {
					continue;
				}

				if ny >= self.size_y {
					continue;
				}

				if self.is_mine(nx, ny) {
					mines += 1;
					continue;
				}

				let state = self.board[self.coordinate_to_idx(nx, ny)];
				if state != 0 {
					continue;
				}

				if mines == 0 { empty_spaces.push([nx,ny]); }

				
				self.render();
				sleep(Duration::from_millis(10));
			}
		}

		self.board[self.coordinate_to_idx(x, y)] = mines+1;

		if mines == 0 {
			for space in empty_spaces {
				self.reveal(space[0], space[1], false);
			}
		}
	}

}

fn clear() -> std::io::Result<()> {
    stdout()
        .execute(cursor::MoveTo(0, 0))? // Move to top-left
		.execute(Clear(ClearType::All))?
		.execute(Clear(ClearType::Purge))?;
    Ok(())
}

fn get_char(idx: u8) -> char {
	if idx < 1 { return '.' };
	if idx == 9 { return 'i' };
	if idx == 10 { return '+' };
	if idx == 11 { return '?' };
	return (idx + 47) as char;
}

fn prompt_u8(prompt: &str) -> u8 {
	loop {
		let mut result: String = String::new();
		println!("{prompt}");
		
		match io::stdin().read_line(&mut result) {
			Ok(_) => (),
			Err(e) => {
				println!("input error: {e}");
				continue;
			}
		}

		match result.trim().parse::<u8>() {
			Ok(value) => break value,
			Err(e) => {
				println!("parse error: {e}");
				continue;
			}
		};
	}
}

fn main() {
	// let size_x: u16 = prompt_u8("Cols:") as u16;
	// let size_y: u16 = prompt_u8("Rows:") as u16;

    let size_x: u16 = 20;
	let size_y: u16 = 20;

	let mut session = Session::new(size_x, size_y);
	session.reveal(0, 0, true);
	session.render();
	println!("Hello, world!");

	1;
}
