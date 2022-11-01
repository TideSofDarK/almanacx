use cgmath::{Matrix3, Matrix4, Rad, SquareMatrix, Vector3};
use common::{
    math::lerp,
    platform::input::{Input, InputCode},
};

pub struct Player {
    walk_speed: f32,
    turn_speed: f32,

    pub pitch: f32,
    pub yaw: f32,
    pub pos: Vector3<f32>,
    pub view: Matrix4<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            walk_speed: 1.45,
            turn_speed: 1.45,

            pitch: 0.0,
            yaw: std::f32::consts::PI,
            pos: Vector3::new(0.0, 0.5, 0.0),
            view: Matrix4::identity(),
        }
    }

    pub fn update(&mut self, dt: f32, input: &Input) {
        if input.is_pressed(InputCode::Home) {
            self.pitch = 0.0;
        }

        let look_up = input.is_held(InputCode::PageUp);
        let look_down = input.is_held(InputCode::PageDown);

        if input.is_held(InputCode::RMB) {
            self.yaw -= input.mouse_raw_delta_x as f32 * 1.16 * dt;
            self.pitch -= input.mouse_raw_delta_y as f32 * 1.16 * dt;
        } else if look_up ^ look_down {
            if look_up {
                self.pitch += dt * self.turn_speed;
            }
            if look_down {
                self.pitch -= dt * self.turn_speed;
            }
        } else {
            self.pitch = lerp(self.pitch, 0.0, dt * 4.0);
        }

        self.pitch = self.pitch.clamp(-0.8, 0.8);

        let turn_left = input.is_held(InputCode::Q) || input.is_held(InputCode::Left);
        let turn_right = input.is_held(InputCode::E) || input.is_held(InputCode::Right);

        if turn_left ^ turn_right {
            if turn_left {
                self.yaw += dt * self.turn_speed;
            }
            if turn_right {
                self.yaw -= dt * self.turn_speed;
            }
        }

        let mut walk_sprint_modifier = self.walk_speed * dt;
        if input.is_held(InputCode::Shift) {
            walk_sprint_modifier *= 1.5;
        }

        let walk_forward = input.is_held(InputCode::W);
        let walk_backward = input.is_held(InputCode::S);

        if walk_forward ^ walk_backward {
            if walk_forward {
                self.pos += Matrix3::from_angle_y(Rad(self.yaw))
                    * -Vector3::unit_z()
                    * walk_sprint_modifier;
            }
            if walk_backward {
                self.pos -= Matrix3::from_angle_y(Rad(self.yaw))
                    * -Vector3::unit_z()
                    * walk_sprint_modifier;
            }
        }

        let walk_left = input.is_held(InputCode::A);
        let walk_right = input.is_held(InputCode::D);

        if walk_left ^ walk_right {
            if walk_left {
                self.pos += Matrix3::from_angle_y(Rad(self.yaw))
                    * -Vector3::unit_x()
                    * walk_sprint_modifier;
            }
            if walk_right {
                self.pos -= Matrix3::from_angle_y(Rad(self.yaw))
                    * -Vector3::unit_x()
                    * walk_sprint_modifier;
            }
        }

        self.view = Matrix4::from_angle_x(Rad(-self.pitch))
            * Matrix4::from_angle_y(Rad(-self.yaw))
            * Matrix4::from_translation(-self.pos);
    }
}
