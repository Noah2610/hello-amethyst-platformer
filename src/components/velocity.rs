use super::component_prelude::*;

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    max:   Option<(f32, f32)>,
}

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, max: None }
    }

    pub fn with_max_velocity(mut self, max: (f32, f32)) -> Self {
        self.max = Some(max);
        self
    }

    pub fn set_x(&mut self, val: f32) {
        self.x = val;
        self.handle_max_velocity();
    }
    pub fn set_y(&mut self, val: f32) {
        self.y = val;
        self.handle_max_velocity();
    }

    pub fn incr_x(&mut self, incr: f32) {
        self.x += incr;
        self.handle_max_velocity();
    }
    pub fn incr_y(&mut self, incr: f32) {
        self.y += incr;
        self.handle_max_velocity();
    }

    fn handle_max_velocity(&mut self) {
        if let Some(max) = self.max {
            if self.x.is_sign_positive() {
                self.x = self.x.min(max.0)
            } else if self.x.is_sign_negative() {
                self.x = self.x.max(-max.0)
            }
            if self.y.is_sign_positive() {
                self.y = self.y.min(max.1)
            } else if self.y.is_sign_negative() {
                self.y = self.y.max(-max.1)
            }
        }
    }
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            x:   0.0,
            y:   0.0,
            max: None,
        }
    }
}
