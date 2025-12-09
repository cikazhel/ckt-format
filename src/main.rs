use std::io;
use rand::Rng;
use ansi_term;

struct session {
    mines: Box<[u8]>,
    board: Vec<u8>
}

fn clear() {
    print!("{}[H{}[2J", 27 as char, 27 as char);
}

fn coordinate_to_idx(size_x: u16, x: u16, y: u16) -> usize {
    return (x + y * size_x) as usize;
}

fn get_char(idx: u8) -> char {
    if idx < 1 { return '.' };
    if idx == 9 { return 'i' };
    if idx == 10 { return '+' };
    return (idx + 48) as char;
}

fn is_mine(mines: &Vec<u8>, size_x: u16, x: u16, y: u16) -> bool {
    let idx = coordinate_to_idx(size_x, x, y);
    let idx_byte = idx/8;
    let chunk = mines[idx_byte as usize];
    // println!("the chunk is {:08b} and index is {}", chunk, idx % 8);
    return chunk & (1 << (idx % 8)) != 0;
}

fn reveal(size_x: u16, board: &[u8], mines: &Vec<u8>, x: u16, y: u16) {
    
}

fn render(size_x: u16, size_y: u16, board: &[u8], mines: &Vec<u8>) {
    clear();
    for y in 0..size_y {
        print!("[");
        for x in 0..size_x {
            let state = board[coordinate_to_idx(size_x, x, y) as usize];
            let display: char = get_char(board[coordinate_to_idx(size_x, x, y) as usize]);
            if is_mine(mines, size_x, x, y) == true {
                print!(" + ");
                continue;
            }
            print!(" {display} ");
        }
        print!("]\n");
    }
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
    let mut randgen: rand::prelude::ThreadRng = rand::rng();

    match ansi_term::enable_ansi_support() {
        Ok(_) => (),
        Err(e) => {
            println!("enable_ansi_support failed; minesweeper will be in regular text (error {e})");
        }
    };

    let size_x: u16 = prompt_u8("Cols:") as u16;
    let size_y: u16 = prompt_u8("Rows:") as u16;

    // let size_x: u8 = 4;
    // let size_y: u8 = 4;

    let size: u16 = size_x*size_y;
    let size_bytes: u16 = (size+8-1)/8;

    let mut mines: Vec<u8> = vec![0; size_bytes as usize];

    // place 20 mines
    for _ in 0..20 {
        let idx_byte: usize = randgen.random_range(0..mines.len());
        let idx_mask: u8 = 1 << randgen.random_range(0..8);
        let state: u8 = mines[idx_byte];
        mines[idx_byte] = state | idx_mask;
    }

    let board: Box<[u8]> = (vec![0; size as usize]).into_boxed_slice();

    render(size_x, size_y, &board, &mines);
    println!("Hello, world!");

    1;
}
