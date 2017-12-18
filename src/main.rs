extern crate cgmath;
#[macro_use]
extern crate glium;

use std::path::Path;
use std::default::Default;

use glium::{glutin, Surface};

mod support;

// This code is essentially straight from the glium teapot example
fn main() {
    let model_file = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("Usage: ./exe model_file"),
    };

    // building the display, ie. the main object
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    // building the vertex and index buffers
    let (mut vertex_buffer, mut scale) = support::load_wavefront(&display, Path::new(&model_file));

    // the program
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 normal;
                out vec3 v_position;
                out vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 140

                in vec3 v_normal;
                out vec4 f_color;

                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    f_color = vec4(color, 1.0);
                }
            ",
        },

        110 => {
            vertex: "
                #version 110

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                attribute vec3 position;
                attribute vec3 normal;
                varying vec3 v_position;
                varying vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 110

                varying vec3 v_normal;

                const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    gl_FragColor = vec4(color, 1.0);
                }
            ",
        },

        100 => {
            vertex: "
                #version 100

                uniform lowp mat4 persp_matrix;
                uniform lowp mat4 view_matrix;

                attribute lowp vec3 position;
                attribute lowp vec3 normal;
                varying lowp vec3 v_position;
                varying lowp vec3 v_normal;

                void main() {
                    v_position = position;
                    v_normal = normal;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * 0.005, 1.0);
                }
            ",

            fragment: "
                #version 100

                varying lowp vec3 v_normal;

                const lowp vec3 LIGHT = vec3(-0.2, 0.8, 0.1);

                void main() {
                    lowp float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);
                    lowp vec3 color = (0.3 + 0.7 * lum) * vec3(1.0, 1.0, 1.0);
                    gl_FragColor = vec4(color, 1.0);
                }
            ",
        },
    ).unwrap();

    let mut camera = support::camera::CameraState::new();
    let mut mouse_pressed = [false; 3];
    let mut mouse_pos = (0.0, 0.0);


    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            ..Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target
            .draw(
                &vertex_buffer,
                &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                &program,
                &uniforms,
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        let mut action = support::Action::Continue;

        // polling and handling the events received by the window
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => action = support::Action::Stop,
                ev => camera.process_input(&ev),
            },
            _ => (),
        });

        action
    });

    /*
    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            scaling: scale,
            eye_pos: camera.get_position(),
            light_dir: camera.get_direction(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        let mut target = display.draw();

        // drawing a frame
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();

        target.finish().unwrap();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    return support::Action::Stop;
                },
                glutin::Event::MouseMoved(x, y) => {
                    mouse_pos = (x as f32, y as f32);
                },
                glutin::Event::MouseInput(state, glutin::MouseButton::Left) => {
                    mouse_pressed[0] = state == glutin::ElementState::Pressed;
                },
                glutin::Event::MouseInput(state, glutin::MouseButton::Right) => {
                    mouse_pressed[1] = state == glutin::ElementState::Pressed;
                },
                glutin::Event::MouseInput(state, glutin::MouseButton::Middle) => {
                    mouse_pressed[2] = state == glutin::ElementState::Pressed;
                },
                glutin::Event::DroppedFile(path) => {
                    println!("Dropped file {}", path.display());
                    match path.extension() {
                        Some(ext) if ext == "obj" => {
                            let load = support::load_wavefront(&display, path.as_path());
                            vertex_buffer = load.0;
                            scale = load.1;
                        },
                        _ => println!("Invalid file"),
                    }
                },
                ev => camera.process_input(&ev),
            }
        }

        support::Action::Continue
    });
*/
}
