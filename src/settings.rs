pub mod prelude {
    pub use super::Settings;
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub view_dimensions:      (f32, f32),
    pub player_size:          (f32, f32),
    pub player_speed:         (f32, f32),
    pub player_jump_strength: f32,
    pub player_max_velocity:  (f32, f32),
    pub player_decr_velocity: (f32, f32),
}
