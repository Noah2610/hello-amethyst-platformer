use super::system_prelude::*;

pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            time,
            input,
            collisions,
            mut players,
            mut velocities,
            mut decr_velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        for (player, velocity, mut decr_velocity, collision) in (
            &mut players,
            &mut velocities,
            (&mut decr_velocities).maybe(),
            &collisions,
        )
            .join()
        {
            // Move left/right, on X axis
            if let Some(x) = input.axis_value("player_x") {
                if x != 0.0 {
                    velocity.x += (player.speed.0 * dt) * (x as f32).signum();
                    decr_velocity.as_mut().map(|decr| {
                        if x > 0.0 {
                            decr.should_decrease_x_pos = false
                        } else if x < 0.0 {
                            decr.should_decrease_x_neg = false
                        }
                    });
                }
            }

            // Jump
            if let Some(is_action_down) = input.action_is_down("player_jump") {
                if is_action_down && !player.is_jump_button_down {
                    velocity.y += settings.player_jump_strength;
                }
                player.is_jump_button_down = is_action_down;
            }

            if collision.in_collision() {
                for (entity, other_collision) in (&entities, &collisions).join()
                {
                    if collision.in_collision_with(entity.id()) {
                        println!("PLAYER IS IN COLLISION!!!");
                    }
                }
            }
        }
    }
}
