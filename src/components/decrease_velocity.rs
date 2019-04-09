use super::component_prelude::*;

#[derive(Deserialize)]
pub struct DecreaseVelocity {
    pub x: f32,
    pub y: f32,
    /// Should decrease X velocity, when X velocity is POSITIVE
    pub should_decrease_x_pos: bool,
    /// Should decrease X velocity, when X velocity is NEGATIVE
    pub should_decrease_x_neg: bool,
    /// Should decrease Y velocity, when X velocity is POSITIVE
    pub should_decrease_y_pos: bool,
    /// Should decrease Y velocity, when X velocity is NEGATIVE
    pub should_decrease_y_neg: bool,
}

impl DecreaseVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            should_decrease_x_pos: false,
            should_decrease_x_neg: false,
            should_decrease_y_pos: false,
            should_decrease_y_neg: false,
        }
    }
}

impl Component for DecreaseVelocity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for DecreaseVelocity {
    fn from(data: (f32, f32)) -> Self {
        Self::new(data.0, data.1)
    }
}
