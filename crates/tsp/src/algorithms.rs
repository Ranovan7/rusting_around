use crate::utils::{ euclidean_distance };
use std::cmp;

pub fn simmulated_annealing() {
    println!("Simmulated Annealing");
}

pub fn should_edges_swap(
    routes: &mut Vec<(i32, i32)>,
    index_a: usize,
    index_b: usize
) -> bool {
    let prev_a = (((index_a as i32 + routes.len() as i32) - 1) % routes.len() as i32) as usize;
    let prev_b = (((index_b as i32 + routes.len() as i32) - 1) % routes.len() as i32) as usize;
    let current = (
        euclidean_distance(&routes[index_a], &routes[prev_a]),
        euclidean_distance(&routes[index_b], &routes[prev_b])
    );
    let swapped = (
        euclidean_distance(&routes[index_a], &routes[index_b]),
        euclidean_distance(&routes[prev_a], &routes[prev_b])
    );

    match (swapped.0 + swapped.1) < (current.0 + current.1) {
        true => {
            let lower = cmp::min(index_a, index_b);
            let higher = cmp::max(index_a, index_b);

            routes[lower..higher].reverse();

            true
        },
        false => false
    }
}
