use deathframe::geo::Vector;

use super::component_prelude::*;

pub struct Player {
    pub speed:               Vector,
    pub max_velocity:        (Option<f32>, Option<f32>),
    pub run_max_velocity:    (Option<f32>, Option<f32>),
    pub is_jump_button_down: bool,
    pub is_run_button_down:  bool,
}

impl Player {
    pub fn new() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub fn with_speed(speed: (f32, f32)) -> Self {
        Self {
            speed,
            is_jump_button_down: false,
            is_run_button_down: false,
            ..Self::default()
        }
    }
}

pub struct PlayerBuilder {
    speed:            Option<Vector>,
    max_velocity:     Option<(Option<f32>, Option<f32>)>,
    run_max_velocity: Option<(Option<f32>, Option<f32>)>,
}

impl PlayerBuilder {
    pub fn speed(mut self, speed: Vector) -> Self {
        self.speed = Some(speed);
        self
    }

    pub fn max_velocity(
        mut self,
        max_velocity: (Option<f32>, Option<f32>),
    ) -> Self {
        self.max_velocity = Some(max_velocity);
        self
    }

    pub fn run_max_velocity(
        mut self,
        run_max_velocity: (Option<f32>, Option<f32>),
    ) -> Self {
        self.run_max_velocity = Some(run_max_velocity);
        self
    }

    pub fn build(self) -> Player {
        let default = Player::default();
        let speed = self.speed.unwrap_or(default.speed);
        let run_max_velocity =
            self.run_max_velocity.unwrap_or(default.run_max_velocity);
        Player {
            speed,
            run_max_velocity,
            ..default
        }
    }
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        Self {
            speed:            None,
            max_velocity:     None,
            run_max_velocity: None,
        }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed:               (1500.0, 0.0),
            max_velocity:        (Some(400.0), None),
            run_max_velocity:    (Some(800.0), None),
            is_jump_button_down: false,
            is_run_button_down:  false,
        }
    }
}
