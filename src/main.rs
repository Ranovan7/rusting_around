use std::env;
use tsp::travelling_salesman;

fn main() {
    let mut args = env::args();

    args.next();    // ignoring the name of the program

    let command = match args.next() {
        Some(arg) => arg,
        None => "error".to_string(),
    };

    match &command[..] {
        "tsp" => travelling_salesman(args),
        "error" => println!("no command found!"),
        _ => println!("no program linked with such command!"),
    }
}
