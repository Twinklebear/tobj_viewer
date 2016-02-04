#[macro_use]
extern crate glium;
extern crate cgmath;

use std::path::Path;
use std::default::Default;

use glium::{glutin, Surface, DisplayBuild};

mod support;

struct State {
    show_app_metrics: bool,
    show_app_main_menu_bar: bool,
    show_app_console: bool,
    show_app_layout: bool,
    show_app_long_text: bool,
    show_app_auto_resize: bool,
    show_app_fixed_overlay: bool,
    show_app_custom_rendering: bool,
    show_app_manipulating_window_title: bool,
    show_app_about: bool,
    no_titlebar: bool,
    no_border: bool,
    no_resize: bool,
    no_move: bool,
    no_scrollbar: bool,
    no_collapse: bool,
    no_menu: bool,
    bg_alpha: f32,
    auto_resize_state: AutoResizeState,
    file_menu: FileMenuState
}

impl Default for State {
    fn default() -> Self {
        State {
            show_app_metrics: false,
            show_app_main_menu_bar: false,
            show_app_console: false,
            show_app_layout: false,
            show_app_long_text: false,
            show_app_auto_resize: false,
            show_app_fixed_overlay: false,
            show_app_custom_rendering: false,
            show_app_manipulating_window_title: false,
            show_app_about: false,
            no_titlebar: false,
            no_border: true,
            no_resize: false,
            no_move: false,
            no_scrollbar: false,
            no_collapse: false,
            no_menu: false,
            bg_alpha: 0.65,
            auto_resize_state: Default::default(),
            file_menu: Default::default()
        }
    }
}

struct FileMenuState {
    enabled: bool
}

impl Default for FileMenuState {
    fn default() -> Self {
        FileMenuState {
            enabled: true
        }
    }
}

struct AutoResizeState {
    lines: i32
}

impl Default for AutoResizeState {
    fn default() -> Self {
        AutoResizeState {
            lines: 10
        }
    }
}

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
    let (mut vertex_buffer, mut scale) = support::load_wavefront(&display, &Path::new(&model_file));

    // the program
    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                uniform mat4 persp_matrix;
                uniform mat4 view_matrix;
                uniform float scaling;

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
                    gl_Position = persp_matrix * view_matrix * vec4(v_position * scaling, 1.0);
                }}
            ",
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

    let mut camera = support::camera::CameraState::new();
    let mut state = State::default();
    let mut opened = true;
    let mut mouse_pressed = [false; 3];
    let mut mouse_pos = (0.0, 0.0);

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
        let (width, height) = target.get_dimensions();

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
                glutin::Event::MouseMoved((x, y)) => {
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
}

