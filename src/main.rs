use std::io;

fn main() {
    // Get latest on lines buffer
    let stdin_iter = io::stdin().lines().enumerate().

    loop {
        print!(">");
        // println!("> {}: {}",  i, line.unwrap());
        let input = stdin_iter.next();
    }
}
