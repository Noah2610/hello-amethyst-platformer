use super::component_prelude::*;

#[derive(Deserialize)]
pub struct DecreaseVelocity {
    pub x: f32,
    pub y: f32,
    /// Should decrease X velocity, when X velocity is POSITIVE
    #[serde(default)]
    pub should_decrease_x_pos: bool,
    /// Should decrease X velocity, when X velocity is NEGATIVE
    #[serde(default)]
    pub should_decrease_x_neg: bool,
    /// Should decrease Y velocity, when X velocity is POSITIVE
    #[serde(default)]
    pub should_decrease_y_pos: bool,
    /// Should decrease Y velocity, when X velocity is NEGATIVE
    #[serde(default)]
    pub should_decrease_y_neg: bool,
}

impl DecreaseVelocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            should_decrease_x_pos: true,
            should_decrease_x_neg: true,
            should_decrease_y_pos: true,
            should_decrease_y_neg: true,
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

impl Default for DecreaseVelocity {
    fn default() -> Self {
        Self {
            x:                     0.0,
            y:                     0.0,
            should_decrease_x_pos: true,
            should_decrease_x_neg: true,
            should_decrease_y_pos: true,
            should_decrease_y_neg: true,
        }
    }
}
