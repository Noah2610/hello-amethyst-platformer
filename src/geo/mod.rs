mod axis;
mod collision;
mod side;

pub mod prelude {
    pub use super::Axis;
    pub use super::Side;
    pub use super::Vector;
    pub use super::{CollisionGrid, CollisionRect};
}

pub use axis::Axis;
pub use collision::{CollisionGrid, CollisionRect};
pub use side::Side;

pub type Vector = (f32, f32);
