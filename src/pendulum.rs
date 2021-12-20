pub struct Bob {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
    pub length: f32,
    pub angle: f32,
    pub angle_velocity: f32,
    pub angle_acceleration: f32
}

pub struct Pendulum {
    pub origin_x: f32,
    pub origin_y: f32,
    pub gravity: f32,

    pub top: Bob,
    pub bottom: Bob
}

impl Pendulum {
    pub fn new(origin_x: f32, origin_y: f32, gravity: f32, top_mass: f32, top_length: f32, top_angle: f32, bottom_mass: f32, bottom_length: f32, bottom_angle: f32) -> Self {
        let top_x: f32 = top_length * top_angle.sin() + origin_x;
        let top_y: f32 = top_length * -1.0 * top_angle.cos() + origin_y;
        let top: Bob = Bob { x: top_x, y: top_y, mass: top_mass, length: top_length, angle: top_angle, angle_velocity: 0.0, angle_acceleration: 0.0};

        let bottom_x: f32 = top_x + bottom_length * bottom_angle.sin();
        let bottom_y: f32 = top_y + bottom_length * -1.0 * bottom_angle.cos();
        let bottom: Bob = Bob { x: bottom_x, y: bottom_y, mass: bottom_mass, length: bottom_length, angle: bottom_angle, angle_velocity: 0.0, angle_acceleration: 0.0};

        Pendulum {origin_x, origin_y, gravity, top, bottom}
    }

    pub fn update(&mut self, time_step: f32) {
        self.top.angle_acceleration = self.calc_top(self.top.angle, self.bottom.angle);
        self.bottom.angle_acceleration = self.calc_bottom(self.top.angle, self.bottom.angle);

        self.top.angle_velocity += self.top.angle_acceleration * time_step;
        self.bottom.angle_velocity += self.bottom.angle_acceleration * time_step;
        
        self.top.angle += self.top.angle_velocity;
        self.bottom.angle += self.bottom.angle_velocity;

        self.polar_to_cartesian();
    }

    fn calc_top(&mut self, a1: f32, a2: f32) -> f32 {
        let g = self.gravity;
        let m1 = self.top.mass;
        let m2 = self.bottom.mass;
        let av1 = self.top.angle_velocity;
        let av2 = self.bottom.angle_velocity;
        let l1 = self.top.length;
        let l2 = self.bottom.length;

        let a = -1.0 * g * (2.0 * m1 + m2) * a1.sin() - m2 * g * (a1 - 2.0 * a2).sin() - 2.0 * (a1 - a2).sin() * m2 * (av2 * l2 + av1 * l1 * (a1-a2).cos());
        let b = l1 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a1).cos());
        return a / b;
    }

    fn calc_bottom(&mut self, a1: f32, a2: f32) -> f32 {
        let g = self.gravity;
        let m1 = self.top.mass;
        let m2 = self.bottom.mass;
        let av1 = self.top.angle_velocity;
        let av2 = self.bottom.angle_velocity;
        let l1 = self.top.length;
        let l2 = self.bottom.length;

        let c = 2.0 * (a1 - a2).sin() * (av1 * l1 * (m1 + m2) + g * (m1 + m2) * a1.cos() + av2 * l2 * m2 * (a1-a2).cos());
        let d = l2 * (2.0 * m1 + m2 - m2 * (2.0 * a1 - 2.0 * a2).cos());
        return c / d;
    }

    fn polar_to_cartesian(&mut self) {
        self.top.x = self.top.length * self.top.angle.sin() + self.origin_x;
        self.top.y = self.top.length * -1.0 * self.top.angle.cos() + self.origin_y;

        self.bottom.x = self.top.x + self.bottom.length * self.bottom.angle.sin();
        self.bottom.y = self.top.y + self.bottom.length * -1.0 * self.bottom.angle.cos();
    }
}