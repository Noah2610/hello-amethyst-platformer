use deathframe::geo::Vector;

pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsCamera;
    pub use super::SettingsPlayer;
    pub use super::SettingsPlayerQuickTurnaround;
}

// TODO: Refactor this. Less fields; more structs.
#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub camera: SettingsCamera,
    pub player: SettingsPlayer,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SettingsCamera {
    pub size:       Vector,
    pub inner_size: Vector,
    pub base_speed: Vector,
    pub deadzone:   Vector,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SettingsPlayer {
    pub size:                 Vector,
    pub acceleration:         Vector,
    pub run_acceleration:     Vector,
    pub jump_strength:        f32,
    pub max_velocity:         (Option<f32>, Option<f32>),
    pub run_max_velocity:     (Option<f32>, Option<f32>),
    pub decr_velocity:        Vector,
    pub gravity:              Vector,
    pub jump_gravity:         Vector,
    pub slide_strength:       f32,
    pub quick_turnaround:     SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround: SettingsPlayerQuickTurnaround,
}

#[derive(Debug, Clone, Copy)]
pub enum SettingsPlayerQuickTurnaround {
    No,             // 0
    ResetVelocity,  // 1
    InvertVelocity, // 2
}

use std::fmt;

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer};

struct QTAVisitor;

impl<'de> Visitor<'de> for QTAVisitor {
    type Value = SettingsPlayerQuickTurnaround;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer between 0 and 2 (inclusive)")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use SettingsPlayerQuickTurnaround as QTA;
        match value {
            0 => Ok(QTA::No),
            1 => Ok(QTA::ResetVelocity),
            2 => Ok(QTA::InvertVelocity),
            _ => Err(E::custom(format!("Value out of range: {}", value))),
        }
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }
}

impl<'de> Deserialize<'de> for SettingsPlayerQuickTurnaround {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(QTAVisitor)
    }
}
