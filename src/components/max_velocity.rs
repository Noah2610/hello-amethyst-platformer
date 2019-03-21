use super::component_prelude::*;

pub struct MaxVelocity {
    pub x: Option<f32>,
    pub y: Option<f32>,
}

impl MaxVelocity {
    pub fn with_xy(x: f32, y: f32) -> Self {
        Self {
            x: Some(x),
            y: Some(y),
        }
    }
    pub fn with_x(x: f32) -> Self {
        Self {
            x: Some(x),
            y: None,
        }
    }
    pub fn with_y(y: f32) -> Self {
        Self {
            x: None,
            y: Some(y),
        }
    }
}

impl Component for MaxVelocity {
    type Storage = VecStorage<Self>;
}

impl From<(f32, f32)> for MaxVelocity {
    fn from(data: (f32, f32)) -> Self {
        Self::with_xy(data.0, data.1)
    }
}

impl From<(Option<f32>, Option<f32>)> for MaxVelocity {
    fn from(data: (Option<f32>, Option<f32>)) -> Self {
        Self {
            x: data.0,
            y: data.1,
        }
    }
}
