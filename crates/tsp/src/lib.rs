mod utils;
mod algorithms;

use std::env;
use std::process;

use utils::{
    Config,
    generate_cities,
    get_route_distance,
    get_all_possible_pairings,
    check_swap_viability,
    create_plot,
    animate_plot
};
use algorithms::{ should_edges_swap };

pub fn travelling_salesman(args: env::Args) {
    let config = Config::new(args).unwrap();

    println!("Traveling Salesman Problem");

    if config.n_city <= 3 {
        println!("City samples too few!");
        process::exit(1);
    }

    let mut routes = generate_cities(&config);
    let mut plots = vec![];

    println!("Current Distance : {}", get_route_distance(&routes));
    println!("Calculating...");

    // let mut i = 0;
    loop {
        // i += 1;
        let mut swapped = false;
        let pairings = get_all_possible_pairings(&routes);

        for pair in &pairings {
            if check_swap_viability(pair.0, pair.1, routes.len()) {
                swapped = should_edges_swap(&mut routes, pair.0, pair.1);

                if swapped {
                    plots.push(create_plot(&routes));
                    break;
                }
            }
        }

        if !swapped {
            break;
        }
    }

    println!("Best Distance Results : {}", get_route_distance(&routes));

    animate_plot(&plots, &config).unwrap();
}
