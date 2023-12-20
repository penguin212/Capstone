#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;

use std::collections::{BinaryHeap, HashSet};


use crate::state::{State,Node};


fn heuristic(state: State, goal: State) -> i32 {
    // println!("{}", (f64::sqrt(((state.x - goal.x).pow(2) + (state.y - goal.y).pow(2)) as f64) * 10.0) as i32 + 1);
    return (f64::sqrt(((state.x - goal.x).pow(2) + (state.y - goal.y).pow(2)) as f64) * 100.0) as i32 + 1;
}

fn cost(state: State, new_state: State) -> i32 {
    return 20 * 100;
}

pub fn calc_path_no_vel(start: State, goal: State) -> Vec<State> {
    
    let mut heap = BinaryHeap::new();
    let mut visited: HashSet<State> = HashSet::new();


    let current_node = Node {
        state: start,
        priority: 0 + heuristic(start, goal),
        path: Vec::<State>::new(),
    };

    heap.push(current_node);

    while let Some(Node { mut state, priority, path }) = heap.pop() {
        
        if(visited.contains(&state)) {
            continue;
        }

        if(state.x < 260 && state.x > 140 && state.y > 140 && state.y < 260) {
            continue;
        }
        visited.insert(state);

        for velocity_change in -1..3 {
            for theta_change in -2..3 {
                let new_state = state.update(velocity_change, theta_change * 7);
                let mut new_path = path.clone();

                // println!("({}, {}, {}, {}) {}", state.x, state.y, state.theta, state.velocity, priority);

                // Just the movement cost to get to the new node
                let new_cost = priority - heuristic(state, goal) + cost(state, new_state); 
                new_path.push(new_state);
                if heuristic(new_state, goal) < 10 {
                    println!("Len: {}", heap.len());
                    return new_path;
                }
                heap.push(Node {
                    state: new_state,
                    priority: new_cost + heuristic(new_state, goal),
                    path: new_path,
                });
            }
        }
    }

    return Vec::<State>::new();
}