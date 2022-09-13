use cgmath::{Point3, Vector3, Rad, Angle, Matrix4, Matrix, SquareMatrix};

pub struct Player {
    angle: f32,
    walk_speed: f32,
    turn_speed: f32,
    view: Matrix4<f32>
}

impl Player {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            walk_speed: 0.05,
            turn_speed: 0.05,
            view: Matrix4::look_to_rh(Point3::new(0.0, 0.0, 0.0),
            Vector3::unit_z(),
            Vector3::unit_y())
        }
    }

    pub fn handle_input(&mut self, forward: bool, backward: bool, left: bool, right: bool, turn_left: bool, turn_right: bool) {
        let sign_z = forward as i16 + -(backward as i16);
        let sign_x = left as i16 + -(right as i16);

        let sign_angle = turn_right as i16 + -(turn_left as i16);

        self.view =  
            Matrix4::from_angle_y(Rad( self.turn_speed * (sign_angle as f32))) *
            Matrix4::from_translation(Vector3::new(sign_x as f32 * self.walk_speed, 0.0, sign_z as f32 * self.walk_speed)) *
            self.view;
    }

    pub fn get_view_matrix(&mut self) -> Matrix4<f32> {
        self.view
    }
}