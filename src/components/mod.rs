mod player;
mod size;
mod velocity;

pub mod prelude {
    pub use amethyst::core::transform::Transform;

    pub use super::Player;
    pub use super::Size;
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

pub use player::Player;
pub use size::Size;
pub use velocity::Velocity;
