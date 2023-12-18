#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::BinaryHeap};

use crate::state::{State,Node};


fn heuristic(state: State, goal: State) -> i32 {
    return f64::sqrt(((state.x - goal.x).pow(2) + (state.y - goal.y).pow(2)) as f64) as i32 + 1;
}

fn cost(state: State, new_state: State) -> i32 {
    return 10;
}

pub fn calc_path_no_vel(start: State, goal: State) -> Vec<State> {
    
    let mut heap = BinaryHeap::new();


    let current_node = Node {
        state: start,
        priority: 0 + heuristic(start, goal),
        path: Vec::<State>::new(),
    };

    heap.push(current_node);

    while let Some(Node { mut state, priority, path }) = heap.pop() {
        if heuristic(state, goal) < 10 {
            println!("Len: {}", heap.len());
            return path;
        }

        // println!("Len: {}", heap.len());

        for velocity_change in -1..3 {
            for theta_change in -1..2 {
                let new_state = state.update(velocity_change, theta_change * 15);

                let mut new_path = path.clone();

                // Just the movement cost to get to the new node
                let new_cost = priority - heuristic(state, goal) + cost(state, new_state); 
                new_path.push(new_state);
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