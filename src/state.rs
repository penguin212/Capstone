#![allow(dead_code)]

use std::f32::consts::PI;
use std::cmp::Ordering;
use std::num;
use std::u128::MAX;

const MAX_VELOCITY: i32 = 20;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub x: i32,
    pub y: i32,
    pub velocity: i32,
    pub theta: i32,
}

impl State {
    pub fn update(&mut self, velocity_change: i32, theta_change: i32) -> State {
        return State {
            x: self.x + ((2.0 * self.velocity as f32 + velocity_change as f32) / 2.0 * (self.theta as f32 * PI / 180.0).cos()) as i32 ,
            y: self.y + ((2.0 * self.velocity as f32 + velocity_change as f32) / 2.0 * (self.theta as f32 * PI / 180.0).sin()) as i32,
            velocity: core::cmp::Ord::clamp(self.velocity + velocity_change, -3, MAX_VELOCITY),
            theta: self.theta + theta_change,
        };
    }
}
#[derive(Clone, Eq, PartialEq)]
pub struct Node {
    pub state: State,
    pub priority: i32,
    pub path: Vec<State>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
            .then_with(|| self.state.x.cmp(&other.state.x))
            .then_with(|| self.state.y.cmp(&other.state.y))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}