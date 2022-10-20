use cgmath::{Matrix4, Point3, Rad, Vector3};

pub struct Player {
    walk_speed: f32,
    turn_speed: f32,

    sign_z: i16,
    sign_x: i16,
    sign_angle: i16,
    sprint: f32,

    view: Matrix4<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            walk_speed: 1.45,
            turn_speed: 1.15,

            sign_z: 0,
            sign_x: 0,
            sign_angle: 0,
            sprint: 1.0,

            view: Matrix4::look_to_lh(
                Point3::new(0.0, 0.5, 0.0),
                Vector3::unit_z(),
                Vector3::unit_y(),
            ),
        }
    }

    pub fn handle_input(
        &mut self,
        forward: bool,
        backward: bool,
        left: bool,
        right: bool,
        turn_left: bool,
        turn_right: bool,
        sprint: bool,
    ) {
        self.sign_z = forward as i16 + -(backward as i16);
        self.sign_x = left as i16 + -(right as i16);

        self.sign_angle = turn_right as i16 + -(turn_left as i16);

        self.sprint = if sprint { 2.0 } else { 1.0 };
    }

    pub fn update(&mut self, dt: f32) {
        self.view = Matrix4::from_angle_y(Rad(dt * self.turn_speed * (self.sign_angle as f32)))
            * Matrix4::from_translation(Vector3::new(
                self.sign_x as f32 * self.walk_speed * dt * self.sprint,
                0.0,
                self.sign_z as f32 * self.walk_speed * dt * self.sprint,
            ))
            * self.view;

        // Consume input
        self.sign_angle = 0;
        self.sign_x = 0;
        self.sign_z = 0;
        self.sprint = 1.0;
    }

    pub fn get_view(&mut self) -> &Matrix4<f32> {
        &self.view
    }
}
