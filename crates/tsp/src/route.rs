use crate::utils::{
    euclidean_distance,
    get_all_possible_pairings
};
use rand::{ thread_rng };
use rand::seq::SliceRandom;
use std::cmp;

pub struct Route {
    pub routes: Vec<(i32, i32)>,
    pairings: Vec<(usize, usize)>
}

impl Route {
    pub fn new(cities: Vec<(i32, i32)>) -> Route {
        let pairings = get_all_possible_pairings(&cities);
        Route {
            routes: cities,
            pairings: pairings
        }
    }

    pub fn total_distance(&self) -> f32 {
        let mut distance = 0.0;
        for (i, route) in self.routes.iter().enumerate() {
            let prev_i = ((i + self.routes.len()) - 1) % self.routes.len();
            distance += euclidean_distance(route, &self.routes[prev_i]);
        }

        distance
    }

    pub fn possible_pairings(&self) -> Vec<(usize, usize)> {
        let mut results = self.pairings.clone();
        results.shuffle(&mut thread_rng());

        results
    }

    pub fn should_edges_swap(&mut self, index_a: usize, index_b: usize) -> bool {
        let viable = self.check_swap_viability(index_a, index_b);

        if !viable {
            false
        } else {
            let prev_a = (((index_a as i32 + self.routes.len() as i32) - 1) % self.routes.len() as i32) as usize;
            let prev_b = (((index_b as i32 + self.routes.len() as i32) - 1) % self.routes.len() as i32) as usize;
            let current = (
                euclidean_distance(&self.routes[index_a], &self.routes[prev_a]),
                euclidean_distance(&self.routes[index_b], &self.routes[prev_b])
            );
            let swapped = (
                euclidean_distance(&self.routes[index_a], &self.routes[index_b]),
                euclidean_distance(&self.routes[prev_a], &self.routes[prev_b])
            );

            match (swapped.0 + swapped.1) < (current.0 + current.1) {
                true => {
                    let lower = cmp::min(index_a, index_b);
                    let higher = cmp::max(index_a, index_b);

                    self.routes[lower..higher].reverse();

                    true
                },
                false => false
            }
        }
    }

    fn check_swap_viability(&self, a: usize, b: usize) -> bool {
        let length = self.routes.len();
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
}
