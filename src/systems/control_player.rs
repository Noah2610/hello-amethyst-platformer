use super::system_prelude::*;

pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (time, input, mut players, mut velocities, mut decr_velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        for (player, velocity, decr_velocity) in
            (&mut players, &mut velocities, &mut decr_velocities).join()
        {
            // Move left/right, on X axis
            if let Some(x) = input.axis_value("player_x") {
                if x != 0.0 {
                    velocity.x += (player.speed.0 * dt) * (x as f32).signum();
                    decr_velocity.should_decrease_x = false;
                } else {
                    decr_velocity.should_decrease_x = true;
                }
            }

            // Jump
            if let Some(is_action_down) = input.action_is_down("player_jump") {
                if is_action_down && !player.is_jump_button_down {
                    velocity.y += constants::PLAYER_JUMP_STRENGTH;
                }
                player.is_jump_button_down = is_action_down;
            }
        }
    }
}
