use deathframe::geo::Vector;

pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsCamera;
    pub use super::SettingsPlayer;
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
    pub size:             Vector,
    pub acceleration:     Vector,
    pub run_acceleration: Vector,
    pub jump_strength:    f32,
    pub max_velocity:     (Option<f32>, Option<f32>),
    pub run_max_velocity: (Option<f32>, Option<f32>),
    pub decr_velocity:    Vector,
    pub gravity:          Vector,
    pub jump_gravity:     Vector,
    pub slide_strength:   f32,
}
