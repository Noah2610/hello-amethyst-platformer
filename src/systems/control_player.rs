use super::system_prelude::*;

pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, input, mut players, mut velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        for (player, velocity) in (&mut players, &mut velocities).join() {
            // Move left/right, on X axis
            if let Some(x) = input.axis_value("player_x") {
                if x != 0.0 {
                    velocity.x += (player.speed.0 * dt) * (x as f32).signum();
                }
            }
        }
    }
}
