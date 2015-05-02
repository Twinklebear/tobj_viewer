#[macro_use]
extern crate glium;
extern crate glutin;
extern crate cgmath;

use std::path::Path;

use cgmath::*;
use glium::Surface;
use glium::DisplayBuild;

mod support;

// This code is essentially straight from the glium teapot example
fn main() {
    let model_file = match std::env::args().nth(1) {
        Some(arg) => arg,
        None => panic!("Usage: ./exe model_file"),
    };

    // building the display, ie. the main object
    let display = glutin::WindowBuilder::new()
        .with_dimensions(1024, 768)
        .with_depth_buffer(24)
        .with_title("tobj model viewer".to_string())
        .with_vsync()
        .with_multisampling(8)
        .build_glium()
        .unwrap();

    // building the vertex and index buffers
    let (vertex_buffer, scale) = support::load_wavefront(&display, &Path::new(&model_file));

    // the program
    let program = program!(&display,
        140 => {
            vertex: &format!("
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;

                in vec3 position;
                in vec3 normal;
                in vec3 color_diffuse;
                in vec4 color_specular;

                out vec3 v_position;
                out vec3 v_normal;
                out vec3 v_color_diffuse;
                out vec4 v_color_specular;

                void main() {{
                    v_position = position;
                    v_normal = normal;
                    v_color_diffuse = color_diffuse;
                    v_color_specular = color_specular;
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * {}, 1.0);
                }}
            ", scale),

            fragment: "
                #version 140

                uniform vec3 eye_pos;
                uniform vec3 light_dir;

                in vec3 v_position;
                in vec3 v_normal;
                in vec3 v_color_diffuse;
                in vec4 v_color_specular;

                out vec4 f_color;

                void main() {
                    vec3 normal = v_normal;
                    // If we don't have normals, use derivative of position to compute
                    if (dot(normal, normal) < 0.001) {
                        normal = normalize(cross(dFdx(v_position), dFdy(v_position)));
                    }
                    vec3 l = normalize(-light_dir);
                    vec3 view_dir = normalize(eye_pos - v_position);
                    float n_dot_l = clamp(dot(normal, l), 0.0, 1.0);
                    vec3 color = (0.1 + n_dot_l * 0.5) * v_color_diffuse;

                    vec3 half_vec = normalize(l + view_dir);
                    float n_dot_h = clamp(dot(normal, half_vec), 0.0, 1.0);
                    if (n_dot_h > 0.0) {
                        color += 0.5 * pow(n_dot_h, v_color_specular.a) * v_color_specular.rgb;
                    }

                    f_color = vec4(color, 1.0);
                }
            ",
        },
    ).unwrap();

    //
    let mut camera = support::camera::CameraState::new();

    // the main loop
    support::start_loop(|| {
        camera.update();

        // building the uniforms
        let uniforms = uniform! {
            persp_matrix: camera.get_perspective(),
            view_matrix: camera.get_view(),
            eye_pos: camera.get_position(),
            light_dir: camera.get_direction(),
        };

        // draw parameters
        let params = glium::DrawParameters {
            depth_test: glium::DepthTest::IfLess,
            depth_write: true,
            .. std::default::Default::default()
        };

        // drawing a frame
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 1.0);
        target.draw(&vertex_buffer,
                    &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program, &uniforms, &params).unwrap();
        target.finish();

        // polling and handling the events received by the window
        for event in display.poll_events() {
            match event {
                glutin::Event::Closed => return support::Action::Stop,
                glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Escape)) => {
                    return support::Action::Stop;
                },
                ev => camera.process_input(&ev),
            }
        }

        support::Action::Continue
    });
}

