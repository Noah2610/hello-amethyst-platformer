use super::system_prelude::*;
use crate::geo::Side;

pub struct ControlPlayerSystem;

impl ControlPlayerSystem {
    fn check_move(
        &self,
        dt: f32,
        input: &Read<InputHandler<String, String>>,
        player: &Player,
        velocity: &mut Velocity,
        mut decr_velocity_opt: Option<&mut DecreaseVelocity>,
    ) {
        // Move left/right, on X axis
        if let Some(x) = input.axis_value("player_x") {
            if x != 0.0 {
                velocity.x += (player.speed.0 * dt) * (x as f32).signum();
                decr_velocity_opt.as_mut().map(|decr| {
                    if x > 0.0 {
                        decr.dont_decrease_x_when_pos();
                    } else if x < 0.0 {
                        decr.dont_decrease_x_when_neg();
                    }
                });
            }
        }
    }
}

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
        WriteStorage<'a, MaxVelocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
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
            mut max_velocities,
            mut decr_velocities,
            mut gravities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        for (
            player,
            mut velocity,
            mut max_velocity_opt,
            decr_velocity_opt,
            collision,
            mut gravity_opt,
        ) in (
            &mut players,
            &mut velocities,
            (&mut max_velocities).maybe(),
            (&mut decr_velocities).maybe(),
            &collisions,
            (&mut gravities).maybe(),
        )
            .join()
        {
            self.check_move(
                dt,
                &input,
                &player,
                &mut velocity,
                decr_velocity_opt,
            );

            // TODO: Refactor the rest of this into their own methods.
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
                    let slide_strength = -settings.player_slide_strength;
                    if velocity.y < slide_strength {
                        velocity.y = slide_strength;
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

            let mut standing_on_floor = false;
            if let Some(side_vert) = touching_vertically_side {
                if let Side::Bottom = side_vert {
                    standing_on_floor = true;
                }
                // Reset y velocity to 0
                if match side_vert {
                    Side::Top => velocity.y > 0.0,
                    Side::Bottom => velocity.y < 0.0,
                    _ => false,
                } {
                    velocity.y = 0.0;
                }
            }

            // Jump
            if let Some(is_jump_down) = input.action_is_down("player_jump") {
                if standing_on_floor
                    && is_jump_down
                    && !player.is_jump_button_down
                {
                    velocity.y += settings.player_jump_strength;
                    gravity_opt.as_mut().map(|gravity| {
                        gravity.x = settings.player_jump_gravity.0;
                        gravity.y = settings.player_jump_gravity.1;
                    });
                } else if !is_jump_down {
                    let decr_jump_strength =
                        settings.player_jump_strength * 0.25;
                    if velocity.y > decr_jump_strength {
                        velocity.y = (velocity.y - decr_jump_strength)
                            .max(decr_jump_strength);
                    }
                    gravity_opt.map(|gravity| {
                        gravity.x = settings.player_gravity.0;
                        gravity.y = settings.player_gravity.1;
                    });
                }
                player.is_jump_button_down = is_jump_down;
            }

            // Run
            if let Some(is_run_down) = input.action_is_down("player_run") {
                max_velocity_opt.as_mut().map(|max_vel| {
                    if is_run_down && !player.is_run_button_down {
                        // Start running
                        max_vel.x = player.run_max_velocity.0;
                        max_vel.y = player.run_max_velocity.1;
                    } else if !is_run_down && player.is_run_button_down {
                        // Stop running
                        max_vel.x = player.max_velocity.0;
                        max_vel.y = player.max_velocity.1;
                    }
                });
                player.is_run_button_down = is_run_down;
            }
        }
    }
}
