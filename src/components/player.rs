use deathframe::geo::Vector;

use super::component_prelude::*;

pub struct Player {
    pub acceleration:        Vector,
    pub run_acceleration:    Vector,
    pub max_velocity:        (Option<f32>, Option<f32>),
    pub run_max_velocity:    (Option<f32>, Option<f32>),
    pub is_jump_button_down: bool,
    pub is_run_button_down:  bool,
}

impl Player {
    pub fn new() -> PlayerBuilder {
        PlayerBuilder::default()
    }

    pub fn with_acceleration(acceleration: (f32, f32)) -> Self {
        Self {
            acceleration,
            is_jump_button_down: false,
            is_run_button_down: false,
            ..Self::default()
        }
    }

    pub fn current_acceleration(&self) -> Vector {
        if self.is_run_button_down {
            self.run_acceleration
        } else {
            self.acceleration
        }
    }
}

pub struct PlayerBuilder {
    acceleration:     Option<Vector>,
    run_acceleration: Option<Vector>,
    max_velocity:     Option<(Option<f32>, Option<f32>)>,
    run_max_velocity: Option<(Option<f32>, Option<f32>)>,
}

impl PlayerBuilder {
    pub fn acceleration(mut self, acceleration: Vector) -> Self {
        self.acceleration = Some(acceleration);
        self
    }

    pub fn run_acceleration(mut self, run_acceleration: Vector) -> Self {
        self.run_acceleration = Some(run_acceleration);
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
        let acceleration = self.acceleration.unwrap_or(default.acceleration);
        let run_max_velocity =
            self.run_max_velocity.unwrap_or(default.run_max_velocity);
        Player {
            acceleration,
            run_max_velocity,
            ..default
        }
    }
}

impl Default for PlayerBuilder {
    fn default() -> Self {
        Self {
            acceleration:     None,
            run_acceleration: None,
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
            acceleration:        (1500.0, 0.0),
            run_acceleration:    (2000.0, 0.0),
            max_velocity:        (Some(400.0), None),
            run_max_velocity:    (Some(800.0), None),
            is_jump_button_down: false,
            is_run_button_down:  false,
        }
    }
}
