mod decrease_velocity;
mod gravity;
mod max_velocity;
mod player;
mod scale;
mod size;
mod solid;
mod velocity;

pub mod prelude {
    pub use amethyst::core::transform::Transform;

    pub use super::DecreaseVelocity;
    pub use super::Gravity;
    pub use super::MaxVelocity;
    pub use super::Player;
    pub use super::Scale;
    pub use super::Size;
    pub use super::Solid;
    pub use super::Velocity;
}

mod component_prelude {
    // NOTE: Quick storage type reference
    // DenseVecStorage: Reduced memory usage for LARGE components.
    // HashMapStorage:  "Best suited for rare components."
    // NullStorage:     Storage without data, used as a simple flag.
    // VecStorage:      Preferable for SMALL components (<= 16 bytes || <= 128 bits). For often used components.
    pub use amethyst::ecs::{
        Component,
        DenseVecStorage,
        HashMapStorage,
        NullStorage,
        Storage,
        VecStorage,
    };

    pub use super::Axis;
}

pub use decrease_velocity::DecreaseVelocity;
pub use gravity::Gravity;
pub use max_velocity::MaxVelocity;
pub use player::Player;
pub use scale::Scale;
pub use size::Size;
pub use solid::Solid;
pub use velocity::Velocity;

#[derive(PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn for_each<C>(mut iterate: C)
    where
        C: FnMut(Self),
    {
        iterate(Axis::X);
        iterate(Axis::Y);
    }

    pub fn is_x(&self) -> bool {
        Axis::X == *self
    }

    pub fn is_y(&self) -> bool {
        Axis::Y == *self
    }
}
