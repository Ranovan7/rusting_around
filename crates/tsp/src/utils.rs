use std::env;
use rand::{ Rng, thread_rng };
use rand::seq::SliceRandom;

pub struct Config {
    pub n_city: i32,
    pub border: i32,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        let n_city: i32 = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => 50,
        };
        let border: i32 = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => 1000,
        };

        Ok(Config {
            n_city,
            border
        })
    }
}

pub fn euclidean_distance(a: &(i32, i32), b: &(i32, i32)) -> f32 {
    ((i32::pow(a.0 - b.0, 2) + i32::pow(a.1 - b.1, 2)) as f32).sqrt()
}

pub fn generate_cities(config: Config) -> Vec<(i32, i32)> {
    let mut results = vec![];
    let mut rng = rand::thread_rng();
    for _ in 1..config.n_city {
        results.push(
            (
                rng.gen_range(0..config.border),
                rng.gen_range(0..config.border)
            )
        );
    }

    results
}

pub fn get_route_distance(routes: &Vec<(i32, i32)>) -> f32 {
    let mut distance = 0.0;
    for (i, route) in routes.iter().enumerate() {
        let prev_i = ((i + routes.len()) - 1) % routes.len();
        distance += euclidean_distance(route, &routes[prev_i]);
    }

    distance
}

pub fn get_all_possible_pairings(
    routes: &Vec<(i32, i32)>
) -> Vec<(usize, usize)> {
    let mut results = vec![];
    for i in 0..routes.len() {
        for j in i+1..routes.len() {
            results.push((i, j))
        }
    }
    results.shuffle(&mut thread_rng());

    results
}

pub fn check_swap_viability(a: usize, b: usize, length: usize) -> bool {
    let last_index = length - 1;
    let diff = a as i32 - b as i32;
    match (a, b) {
        (0, bb) => {
            if bb == last_index {
                false
            } else {
                true
            }
        },
        (aa, 0) => {
            if aa == last_index {
                false
            } else {
                true
            }
        },
        (_, _) => {
            if diff.abs() == 1 {
                false
            } else {
                true
            }
        }
    }
}

pub fn create_plot() {

}

pub fn animate_plot() {

}
