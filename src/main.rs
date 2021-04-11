use std::env;
use tsp::travelling_salesman;
use web_server::web_server;
use ecs::ecs_example;
use boids::boids_simulation;

fn main() {
    let mut args = env::args();

    args.next();    // ignoring the name of the program

    let command = match args.next() {
        Some(arg) => arg,
        None => "error".to_string(),
    };

    match &command[..] {
        "tsp" => travelling_salesman(args),
        "ws" => web_server(),
        "ecs" => ecs_example(),
        "boids" => boids_simulation(),
        "error" => println!("no command found!"),
        _ => println!("no program linked with such command!"),
    }
}
