use glutin;
use cgmath::*;

pub struct CameraState {
    aspect_ratio: f32,
    position: Point3<f32>,
    direction: Point3<f32>,
    perspective: Matrix4<f32>,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        let aspect = 1024.0 / 768.0;
        CameraState {
            aspect_ratio: aspect,
            position: Point3::new(0.1, 0.1, 1.0),
            direction: Point3::new(0.0, 0.0, -1.0),
            perspective: perspective(deg(60.0), aspect, 0.1, 100.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        self.perspective.into_fixed()
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        // Why does this crap ass cgmath library not have any addition operator for Points??? WTF?
        let target = Point3::new(self.position.x + self.direction.x, self.position.y + self.direction.y,
                                 self.position.z + self.direction.z);
        AffineMatrix3::look_at(&self.position, &target, &Vector3::new(0.0, 1.0, 0.0)).mat.into_fixed()
    }

    pub fn update(&mut self) {
        if self.moving_up {
            self.position.y += 0.01;
        }

        if self.moving_left {
            self.position.x -= 0.01;
        }

        if self.moving_down {
            self.position.y -= 0.01;
        }

        if self.moving_right {
            self.position.x += 0.01;
        }

        if self.moving_forward {
            self.position.x += self.direction.x * 0.01;
            self.position.y += self.direction.y * 0.01;
            self.position.z += self.direction.z * 0.01;
        }

        if self.moving_backward {
            self.position.x -= self.direction.x * 0.01;
            self.position.y -= self.direction.y * 0.01;
            self.position.z -= self.direction.z * 0.01;
        }
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Space)) => {
                self.moving_up = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Space)) => {
                self.moving_up = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.moving_down = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = false;
            },
            _ => {}
        }
    }
}

