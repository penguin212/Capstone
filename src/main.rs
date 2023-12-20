#![allow(dead_code)]
#![allow(unused_variables)]

extern crate sdl2;
mod state; 
mod calc_paths_no_vel;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::render::TextureCreator;
use state::State;
use std::time::Duration;


//https://docs.rs/sdl2/latest/sdl2/
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(200, 200, 200));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut theta: i32;
    // let mut velocity: f64 = 0.0;
    let mut x: i32;
    let mut y: i32;
    let mut i: i32 = 0;



    let start_state = state::State {
        x: 100,
        y: 100,
        velocity: 20,
        theta: 0,
    };

    let mut goal_state: State;
    let path: Vec::<state::State>;

    goal_state = state::State {
        x: 150,
        y: 500,
        velocity: 0,
        theta: 0,
    };
    let path = calc_paths_no_vel::calc_path_no_vel(start_state, goal_state);

    // for j in 0..20{
    //     println!("{}", j);
    //     goal_state = state::State {
    //         x: 100 + j * 5,
    //         y: 100 + j * 5,
    //         velocity: 0,
    //         theta: 0,
    //     };
    //     let path = calc_paths_no_vel::calc_path_no_vel(start_state, goal_state);
    // }

    

    for state in path.clone() {
        println!("({}, {}, {}, {})", state.x, state.y, state.theta, state.velocity);
    }

    'running: loop {
        {
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running;
                    },
                    // Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    //     theta += 3.;
                    // },
                    // Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                    //     theta -= 3.;
                    // },
                    // Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                    //     velocity += 3.;
                    // },
                    // Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    //     velocity -= 3.;
                    // },
                    _ => {}
                }
            }
        }

        let current_state: state::State = path.clone()[i as usize];

        x = current_state.x as i32;
        y = current_state.y as i32;
        theta = current_state.theta as i32;

        // println!("{}", current_state.velocity);

        i += 1;
        i = i % path.len() as i32;

        let texture_creator: TextureCreator<_> = canvas.texture_creator();

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        canvas.fill_rect(sdl2::rect::Rect::new(start_state.x, start_state.y, 4, 4)).unwrap();
        canvas.fill_rect(sdl2::rect::Rect::new(goal_state.x, goal_state.y, 4, 4)).unwrap();

        canvas.set_draw_color(Color::RGB(20,20,20));
        canvas.fill_rect(sdl2::rect::Rect::new(150,150,100,100)).unwrap();

        for state in path.clone() {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.fill_rect(sdl2::rect::Rect::new(state.x, state.y, 2, 2)).unwrap();
        }

        let mut texture = texture_creator.create_texture_target(None, 30, 10).unwrap();

        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.set_draw_color(Color::RGB(255, 255, 0)); // Set the color to red
            texture_canvas.fill_rect(sdl2::rect::Rect::new(0, 0, 30, 10)).expect("Could not draw rectangle");
        }).unwrap();

        // Now you can draw the texture with rotation
        let center = None; // Center of rotation
        canvas.copy_ex(&texture, None, Some(sdl2::rect::Rect::new(x, y, 2, 2)), theta as f64, center, false, false).unwrap();
        canvas.set_draw_color(Color::RGB(200, 200, 200));
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 6));
    }
}