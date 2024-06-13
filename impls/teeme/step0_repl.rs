use std::io::{self, Write};

fn main() {
    let mut line = String::new();
    loop {
        print!("user> ");
        io::stdout().flush().unwrap();
        let bytes = io::stdin().read_line(&mut line).expect("could not read line");
        if bytes == 0 {
            println!();
            break;
        }
    }
}

fn read() {
    todo!()
}

fn print() {
    todo!()
}

fn eval() {
    todo!()
}
