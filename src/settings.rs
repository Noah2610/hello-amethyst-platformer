pub mod prelude {
    pub use super::Settings;
}

// TODO: Refactor this. Less fields; more structs.
#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub camera_size:             (f32, f32),
    pub camera_inner_size:       (f32, f32),
    pub camera_base_speed:       (f32, f32),
    pub camera_deadzone:         (f32, f32),
    pub player_size:             (f32, f32),
    pub player_speed:            (f32, f32),
    pub player_run_speed:        (f32, f32),
    pub player_jump_strength:    f32,
    pub player_max_velocity:     (Option<f32>, Option<f32>),
    pub player_run_max_velocity: (Option<f32>, Option<f32>),
    pub player_decr_velocity:    (f32, f32),
    pub player_gravity:          (f32, f32),
    pub player_jump_gravity:     (f32, f32),
    pub player_slide_strength:   f32,
}
