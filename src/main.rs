use std::io;

fn render(size_x: u8, size_y: u8, mines: &[u8]) {
    for y in 0..size_y {
        print!("[");
        for x in 0..size_x {
            let value: u8 = mines[(y*size_x+x) as usize];
            print!(" {value} ");
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
    }
}

fn main() {
    let size_x = prompt_u8("Cols:");
    println!("returned: {size_x}");
    let mines: [u8; 4] = [0; 4];
    render(2, 2, &mines);
    println!("Hello, world!");
}
