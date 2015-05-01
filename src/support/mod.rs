#![allow(dead_code)]

/// support is taken from the [glium](https://github.com/tomaka/glium) examples with only slight
/// modifications, eg. to use tobj for loading models and switching to an arcball camera

extern crate tobj;
extern crate clock_ticks;
extern crate glium;

use std::path::Path;
use std::thread;
use std::f32;

use glium::Display;
use glium::vertex::VertexBufferAny;

pub mod camera;

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F) where F: FnMut() -> Action {
    let mut accumulator = 0;
    let mut previous_clock = clock_ticks::precise_time_ns();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => ()
        };

        let now = clock_ticks::precise_time_ns();
        accumulator += now - previous_clock;
        previous_clock = now;

        const FIXED_TIME_STAMP: u64 = 16666667;
        while accumulator >= FIXED_TIME_STAMP {
            accumulator -= FIXED_TIME_STAMP;
        }

        thread::sleep_ms(((FIXED_TIME_STAMP - accumulator) / 1000000) as u32);
    }
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display, path: &Path) -> (VertexBufferAny, f32) {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        color: [f32; 3],
    }

    implement_vertex!(Vertex, position, normal, color);

    let mut min_pos = [f32::INFINITY; 3];
    let mut max_pos = [f32::NEG_INFINITY; 3];
    let mut vertex_data = Vec::new();
    match tobj::load_obj(path) {
        Ok((models, mats)) => {
            // Just upload the first object in the group
            for model in &models {
                let mesh = &model.mesh;
                println!("Uploading model: {}", model.name);
                for idx in &mesh.indices {
                    let i = *idx as usize;
                    let pos = [mesh.positions[3 * i], mesh.positions[3 * i + 1], mesh.positions[3 * i + 2]];
                    let normal =
                        if !mesh.normals.is_empty() {
                            [mesh.normals[3 * i], mesh.normals[3 * i + 1], mesh.normals[3 * i + 2]]
                        } else {
                            [0.0, 0.0, 0.0]
                        };
                    let color =
                        match mesh.material_id {
                            Some(i) => mats[i].diffuse,
                            None => [1.0, 1.0, 1.0],
                        };
                    vertex_data.push(Vertex {
                        position: pos,
                        normal: normal,
                        color: color,
                    });
                    // Update our min/max pos so we can figure out the bounding box of the object
                    // to view it
                    for i in 0..3 {
                        min_pos[i] = f32::min(min_pos[i], pos[i]);
                        max_pos[i] = f32::max(max_pos[i], pos[i]);
                    }
                }
            }
        },
        Err(e) => panic!("Loading of {:?} failed due to {:?}", path, e),
    }
    // Compute scale factor to fit the model with a [-1, 1] bounding box
    let diagonal_len = 6.0;
    let current_len = f32::powf(max_pos[0] - min_pos[0], 2.0) + f32::powf(max_pos[1] - min_pos[1], 2.0)
        + f32::powf(max_pos[2] - min_pos[2], 2.0);
    let scale = f32::sqrt(diagonal_len / current_len);
    println!("Model scaled by {} to fit", scale);
    (glium::vertex::VertexBuffer::new(display, vertex_data).into_vertex_buffer_any(), scale)
}

