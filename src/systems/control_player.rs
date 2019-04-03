use super::system_prelude::*;
use crate::geo::Side;

pub struct ControlPlayerSystem;

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid>,
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
            solids,
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

            // Is standing on solid? Is touching a solid (horizontally)?
            let mut touching_vertically_side = None;
            let mut touching_horizontally_side = None;
            if collision.in_collision() {
                for (other_entity, _, _) in
                    (&entities, &collisions, &solids).join()
                {
                    if let Some(colliding_with) =
                        collision.collision_with(other_entity.id())
                    {
                        match colliding_with.side {
                            Side::Top | Side::Bottom => {
                                touching_vertically_side =
                                    Some(colliding_with.side)
                            }
                            Side::Left | Side::Right => {
                                touching_horizontally_side =
                                    Some(colliding_with.side)
                            }
                            _ => (),
                        }
                        if touching_vertically_side.is_some()
                            && touching_horizontally_side.is_some()
                        {
                            break;
                        }
                    }
                }
            }

            if let Some(side_hor) = touching_horizontally_side {
                // Reset x velocity to 0
                if match side_hor {
                    Side::Left => velocity.x < 0.0,
                    Side::Right => velocity.x > 0.0,
                    _ => false,
                } {
                    velocity.x = 0.0;
                }
                if touching_vertically_side.is_none() {
                    // Keep (positive/downwards) y velocity at a constant; slide on wall
                    let slide_strength = -10.0; // TODO: put this settings ron file
                    if velocity.y < slide_strength {
                        velocity.y = -10.0;
                    }
                    // Wall Jump
                    if let Some(is_action_down) =
                        input.action_is_down("player_jump")
                    {
                        if is_action_down && !player.is_jump_button_down {
                            if velocity.y < 0.0 {
                                velocity.y = 0.0;
                            }
                            velocity.y += settings.player_jump_strength;
                            // TODO: Have separate `player_wall_jump_strength` setting
                            match side_hor {
                                Side::Left => {
                                    velocity.x += settings.player_jump_strength
                                }
                                Side::Right => {
                                    velocity.x -= settings.player_jump_strength
                                }
                                _ => (),
                            }
                        }
                        player.is_jump_button_down = is_action_down;
                    }
                }
            }

            if let Some(side_vert) = touching_vertically_side {
                // Reset y velocity to 0
                if match side_vert {
                    Side::Top => velocity.y > 0.0,
                    Side::Bottom => velocity.y < 0.0,
                    _ => false,
                } {
                    velocity.y = 0.0;
                }
                // Jump
                if let Side::Bottom = side_vert {
                    if let Some(is_action_down) =
                        input.action_is_down("player_jump")
                    {
                        if is_action_down && !player.is_jump_button_down {
                            velocity.y += settings.player_jump_strength;
                        }
                        player.is_jump_button_down = is_action_down;
                    }
                }
            }
        }
    }
}
