use std::io;
use rand::Rng;
use ansi_term;

fn clear() {
    print!("{}[H{}[2J", 27 as char, 27 as char);
}

fn get_char(idx: u8) -> char {
    if idx < 1 { return '.' };
    if idx == 9 { return 'i' };
    if idx == 10 { return '+' };
    return (idx + 48) as char;
}

fn render(size_x: u8, size_y: u8, board: &[u8]) {
    clear();
    for y in 0..size_y {
        print!("[");
        for x in 0..size_x {
            let display: char = get_char(board[(y*size_x+x) as usize]);

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

fn is_mine(mines: Vec<u8>, size_x: u8, x: u8, y: u8) -> bool {
    let idx: u8 = x + y * size_x;
    let idx_byte: u32 = idx as u32/8;
    println!("checking for mine at chunk {idx_byte}");
    let chunk: u8 = mines[idx_byte as usize];
    println!("the chunk is {:08b} and index is {}", chunk, idx % 8);
    return chunk & (1 << (idx % 8)) == 0;
}

fn main() {
    let mut randgen: rand::prelude::ThreadRng = rand::rng();

    match ansi_term::enable_ansi_support() {
        Ok(_) => (),
        Err(e) => {
            println!("enable_ansi_support failed; minesweeper will be in regular text (error {e})");
        }
    };

    // let size_x: u8 = prompt_u8("Cols:");
    // let size_y: u8 = prompt_u8("Rows:");

    let size_x: u8 = 4;
    let size_y: u8 = 4;

    let size: u32 = size_x as u32*size_y as u32;
    let size_bytes: u32 = (size/8)+1;

    let mut mines: Vec<u8> = vec![0; size_bytes as usize];

    for idx in 0..size_bytes as u32 {
        // let chunk: u8 = 0;
        // // for bit in 0..8 as u8 {
        // //     chunk = chunk | 1 << bit;
        // // }
        // println!("chunk {idx} is equal to {:08b}", chunk);
        mines[idx as usize] = randgen.random::<u8>();
    }

    is_mine(mines, size_x, 0, 3);

    let board: Box<[u8]> = (vec![0; (size_x as u32*size_y as u32) as usize]).into_boxed_slice();

    render(size_x, size_y, &board);
    println!("Hello, world!");
}
