mod check_collision;
pub mod collision;
mod decrease_velocity;
mod gravity;
mod inner_size;
mod max_velocity;
mod player;
mod scale_once;
mod size;
mod solid;
mod velocity;

pub mod prelude {
    pub use amethyst::core::transform::Transform;
    pub use amethyst::renderer::Camera;

    pub use super::collision;
    pub use super::CheckCollision;
    pub use super::Collision;
    pub use super::DecreaseVelocity;
    pub use super::Gravity;
    pub use super::InnerSize;
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

pub use check_collision::CheckCollision;
pub use collision::Collision;
pub use decrease_velocity::DecreaseVelocity;
pub use gravity::Gravity;
pub use inner_size::InnerSize;
pub use max_velocity::MaxVelocity;
pub use player::Player;
pub use scale_once::ScaleOnce;
pub use size::Size;
pub use solid::Solid;
pub use velocity::Velocity;

use amethyst::ecs::EntityBuilder;
use amethyst::prelude::Builder;
use regex::{Match, Regex};

pub fn add_component_by_name<'a>(
    mut entity: EntityBuilder<'a>,
    component_name: &str,
) -> EntityBuilder<'a> {
    let re = Regex::new(r"(?P<name>\w+)(?P<params>\{.*\})?").unwrap();

    if let Some(capture) = re.captures(component_name) {
        match (
            capture.name("name").map(|x| x.as_str()).unwrap_or(""),
            capture.name("params").map(|x| x.as_str()).unwrap_or(""),
        ) {
            ("CheckCollision", _) => {
                entity = entity.with(check_collision::CheckCollision)
            }
            ("Collision", _) => {
                entity = entity.with(collision::Collision::new())
            }
            ("DecreaseVelocity", data) => {
                if let Ok(deserialized) =
                    serde_json::from_str::<DecreaseVelocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for \
                         DecreaseVelocity:\n{:#?}",
                        data
                    ))
                }
            }
            ("Gravity", data) => {
                if let Ok(deserialized) = serde_json::from_str::<Gravity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for Gravity:\n{:#?}",
                        data
                    ))
                }
            }
            ("MaxVelocity", data) => {
                if let Ok(deserialized) =
                    serde_json::from_str::<MaxVelocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for \
                         MaxVelocity:\n{:#?}",
                        data
                    ))
                }
            }
            ("Solid", _) => entity = entity.with(solid::Solid),
            ("Velocity", mut data) => {
                if data.is_empty() {
                    data = "{}"
                }
                if let Ok(deserialized) = serde_json::from_str::<Velocity>(data)
                {
                    entity = entity.with(deserialized);
                } else {
                    panic!(format!(
                        "Couldn't deserialize JSON data for Velocity:\n{:#?}",
                        data
                    ))
                }
            }
            _ => (),
        }
    }

    entity
}
