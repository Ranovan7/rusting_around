mod utils;
mod route;
mod algorithms;

use std::env;
use std::process;

use utils::{
    Config,
    generate_cities,
    create_plot,
    animate_plot
};
use route::Route;

pub fn travelling_salesman(args: env::Args) {
    let config = Config::new(args).unwrap();

    println!("Traveling Salesman Problem");

    if config.n_city <= 3 {
        println!("City samples too few!");
        process::exit(1);
    }

    let cities = generate_cities(&config);
    let mut route = Route::new(cities);
    let mut plots = vec![];

    println!("Current Distance : {}", route.total_distance());
    println!("Calculating...");

    // let mut i = 0;
    loop {
        // i += 1;
        let mut swapped = false;
        let pairings = &route.possible_pairings();

        for pair in pairings {
            swapped = route.should_edges_swap(pair.0, pair.1);

            if swapped {
                plots.push(create_plot(&route.routes));
                break;
            }
        }

        if !swapped {
            break;
        }
    }

    println!("Best Distance Results : {}", route.total_distance());

    animate_plot(&mut plots, &config).unwrap();
}
