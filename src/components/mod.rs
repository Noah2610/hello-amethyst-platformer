pub mod collision;
mod decrease_velocity;
mod gravity;
mod max_velocity;
mod player;
mod scale_once;
mod size;
mod solid;
mod velocity;

pub mod prelude {
    pub use amethyst::core::transform::Transform;

    pub use super::collision;
    pub use super::Collision;
    pub use super::DecreaseVelocity;
    pub use super::Gravity;
    pub use super::MaxVelocity;
    pub use super::Player;
    pub use super::ScaleOnce;
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
}

pub use collision::Collision;
pub use decrease_velocity::DecreaseVelocity;
pub use gravity::Gravity;
pub use max_velocity::MaxVelocity;
pub use player::Player;
pub use scale_once::ScaleOnce;
pub use size::Size;
pub use solid::Solid;
pub use velocity::Velocity;
