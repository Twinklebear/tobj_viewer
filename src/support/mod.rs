#![allow(dead_code)]

extern crate clock_ticks;
extern crate tobj;

use glium::vertex::VertexBufferAny;
use glium::{self, Display};
use std::f32;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

pub mod camera;

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(mut callback: F)
where
    F: FnMut() -> Action,
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667);
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

/// Returns a vertex buffer that should be rendered as `TrianglesList`.
pub fn load_wavefront(display: &Display, path: &Path) -> (VertexBufferAny, f32) {
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 3],
        normal: [f32; 3],
        color_diffuse: [f32; 3],
        color_specular: [f32; 4],
    }

    implement_vertex!(Vertex, position, normal, color_diffuse, color_specular);

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
                    let pos = [
                        mesh.positions[3 * i],
                        mesh.positions[3 * i + 1],
                        mesh.positions[3 * i + 2],
                    ];
                    let normal = if !mesh.normals.is_empty() {
                        [
                            mesh.normals[3 * i],
                            mesh.normals[3 * i + 1],
                            mesh.normals[3 * i + 2],
                        ]
                    } else {
                        [0.0, 0.0, 0.0]
                    };
                    let (color_diffuse, color_specular) = match mesh.material_id {
                        Some(i) => (
                            mats[i].diffuse,
                            [
                                mats[i].specular[0],
                                mats[i].specular[1],
                                mats[i].specular[2],
                                mats[i].shininess,
                            ],
                        ),
                        None => ([0.8, 0.8, 0.8], [0.15, 0.15, 0.15, 15.0]),
                    };
                    vertex_data.push(Vertex {
                        position: pos,
                        normal: normal,
                        color_diffuse: color_diffuse,
                        color_specular: color_specular,
                    });
                    // Update our min/max pos so we can figure out the bounding box of the object
                    // to view it
                    for i in 0..3 {
                        min_pos[i] = f32::min(min_pos[i], pos[i]);
                        max_pos[i] = f32::max(max_pos[i], pos[i]);
                    }
                }
            }
        }
        Err(e) => panic!("Loading of {:?} failed due to {:?}", path, e),
    }
    // Compute scale factor to fit the model with a [-1, 1] bounding box
    let diagonal_len = 6.0;
    let current_len = f32::powf(max_pos[0] - min_pos[0], 2.0)
        + f32::powf(max_pos[1] - min_pos[1], 2.0)
        + f32::powf(max_pos[2] - min_pos[2], 2.0);
    let scale = f32::sqrt(diagonal_len / current_len);
    println!("Model scaled by {} to fit", scale);
    (
        glium::vertex::VertexBuffer::new(display, &vertex_data)
            .unwrap()
            .into_vertex_buffer_any(),
        scale,
    )
}
