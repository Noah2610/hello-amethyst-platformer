use super::system_prelude::*;
use crate::geo::Side;

pub struct ControlPlayerSystem;

impl ControlPlayerSystem {
    /// Returns a tuple with two options:
    /// `(Option<Side>, Option<Side>)`
    /// representing if their is a solid collision on the x axis (horizontally, left/right)
    /// or on the y axis (vertically, top/bottom), and which side is in collision there.
    fn is_touching_solids_on_sides_horizontally_or_vertically<'a>(
        &self,
        entities: &Entities,
        collision: &Collision,
        collisions: &ReadStorage<'a, Collision>,
        solids: &ReadStorage<'a, Solid>,
    ) -> (Option<Side>, Option<Side>) {
        let mut touching_horizontally_side = None;
        let mut touching_vertically_side = None;
        if collision.in_collision() {
            for (other_entity, _, _) in (entities, collisions, solids).join() {
                if let Some(colliding_with) =
                    collision.collision_with(other_entity.id())
                {
                    match colliding_with.side {
                        Side::Top | Side::Bottom => {
                            touching_vertically_side = Some(colliding_with.side)
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
        (touching_horizontally_side, touching_vertically_side)
    }

    fn handle_jump_recharge<'a>(
        &self,
        entities: &Entities<'a>,
        player: &mut Player,
        collision: &Collision,
        collisions: &ReadStorage<'a, Collision>,
        jump_recharges: &ReadStorage<'a, JumpRecharge>,
    ) {
        for (entity, _) in (entities, jump_recharges).join() {
            if let Some(coll_data) = collision.collision_with(entity.id()) {
                if coll_data.side.is_inner() && coll_data.state.is_entering() {
                    player.has_double_jumped = false;
                }
            }
        }
    }

    /// Handle some stuff to do with clinging to a wall (slow slide, wall jump, etc.)
    fn handle_wall_cling(
        &self,
        settings: &Settings,
        input_manager: &Read<InputManager>,
        player: &mut Player,
        velocity: &mut Velocity,
        (touching_horizontally_side, touching_vertically_side): (
            Option<Side>,
            Option<Side>,
        ),
    ) {
        player.is_on_wall = false;
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
                player.is_on_wall = true;
                // Keep (positive/downwards) y velocity at a constant; slide on wall
                let slide_strength = -settings.player.slide_strength;
                if velocity.y < slide_strength {
                    velocity.y = slide_strength;
                }
                // Wall Jump
                if input_manager.is_pressed("player_jump") {
                    if !player.is_jump_button_down {
                        if velocity.y < 0.0 {
                            velocity.y = 0.0;
                        }
                        velocity.y += settings.player.jump_strength;
                        // TODO: Have separate `player_wall_jump_strength` setting
                        match side_hor {
                            Side::Left => {
                                velocity.x += settings.player.jump_strength
                            }
                            Side::Right => {
                                velocity.x -= settings.player.jump_strength
                            }
                            _ => (),
                        }
                    }
                    player.is_jump_button_down = true;
                } else {
                    player.is_jump_button_down = false;
                }
            }
        }
    }

    /// Handle some specifics when player is standing on solid ground vs when they are in air.
    fn handle_on_ground_and_in_air(
        &self,
        player: &mut Player,
        velocity: &mut Velocity,
        (_touching_horizontally_side, touching_vertically_side): (
            Option<Side>,
            Option<Side>,
        ),
    ) {
        player.is_in_air = true;
        if let Some(side_vert) = touching_vertically_side {
            if let Side::Bottom = side_vert {
                // Standing on ground
                player.is_in_air = false;
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
    }

    /// Move player left/right, if necessary
    fn handle_move(
        &self,
        dt: f32,
        settings: &Settings,
        input: &Read<InputHandler<String, String>>,
        player: &Player,
        velocity: &mut Velocity,
        mut decr_velocity_opt: Option<&mut DecreaseVelocity>,
    ) {
        use crate::settings::SettingsPlayerQuickTurnaround as QTA;

        // Move left/right, on X axis
        if let Some(x) = input.axis_value("player_x") {
            let x = x as f32;
            if x != 0.0 {
                let turned_around = x.signum() != velocity.x.signum();
                if turned_around {
                    // Quick turnaround, when on ground
                    let qta_setting = if player.on_ground() {
                        settings.player.quick_turnaround
                    // Quick turnaround, when in air
                    } else {
                        settings.player.air_quick_turnaround
                    };
                    match qta_setting {
                        QTA::ResetVelocity => velocity.x = 0.0,
                        QTA::InvertVelocity => velocity.x *= -1.0,
                        _ => (),
                    }
                }
                velocity.x += (player.current_acceleration().0 * dt)
                    * (x as f32).signum();
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

    /// Handle player jumping. Regular and double jumps.
    fn handle_jump(
        &self,
        settings: &Settings,
        input_manager: &InputManager,
        player: &mut Player,
        velocity: &mut Velocity,
        gravity_opt: &mut Option<&mut Gravity>,
        (audio_handler, audio_source, audio_output): (
            &AudioHandler,
            &AssetStorage<Source>,
            &Output,
        ),
    ) {
        let is_jump_down = input_manager.is_pressed("player_jump");
        let should_jump = (player.on_ground()  // Is standing on ground
                    || (settings.player.is_double_jump_enabled  // Or has double jump available
                        && !player.has_double_jumped))
                    && is_jump_down  // And jump button is currently down
                    && !player.is_jump_button_down; // And jump button has not already been down
        if should_jump {
            // TODO: TEMPORARY.
            // Play sfx.
            if let Some(sfx) = &audio_handler.sfx {
                if let Some(sound) = audio_source.get(sfx) {
                    audio_output.play_once(sound, 1.0);
                }
            }

            player.has_double_jumped = player.in_air();
            if velocity.y < 0.0 {
                velocity.y = 0.0;
            }
            velocity.y += settings.player.jump_strength;
            gravity_opt.as_mut().map(|gravity| {
                gravity.x = settings.player.jump_gravity.0;
                gravity.y = settings.player.jump_gravity.1;
            });
        } else if !is_jump_down {
            let decr_jump_strength = settings.player.jump_strength * 0.25;
            if velocity.y > decr_jump_strength {
                velocity.y =
                    (velocity.y - decr_jump_strength).max(decr_jump_strength);
            }
            gravity_opt.as_mut().map(|gravity| {
                gravity.x = settings.player.gravity.0;
                gravity.y = settings.player.gravity.1;
            });
        }
        player.is_jump_button_down = is_jump_down;

        if player.on_ground() || player.on_wall() {
            player.has_double_jumped = false;
        }
    }

    /// Handle running.
    /// Increase max velocity when holding down run button.
    fn handle_run(
        &self,
        input_manager: &InputManager,
        player: &mut Player,
        max_velocity_opt: &mut Option<&mut MaxVelocity>,
    ) {
        let is_run_down = input_manager.is_pressed("player_run");
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

// TODO: TEMPORARY.
use crate::game::states::startup::AudioHandler;
use amethyst::audio::{output::Output, Source};

impl<'a> System<'a> for ControlPlayerSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        ReadExpect<'a, Output>,
        ReadExpect<'a, AudioHandler>,
        Read<'a, AssetStorage<Source>>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, InputManager>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, JumpRecharge>,
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
            audio_output,
            audio_handler,
            audio_source,
            time,
            input_handler,
            input_manager,
            collisions,
            solids,
            jump_recharges,
            mut players,
            mut velocities,
            mut max_velocities,
            mut decr_velocities,
            mut gravities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();
        for (
            mut player,
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
            // Check if the player is touching solids, and on which sides.
            let (touching_horizontally_side, touching_vertically_side) = self
                .is_touching_solids_on_sides_horizontally_or_vertically(
                    &entities,
                    &collision,
                    &collisions,
                    &solids,
                );

            // Check if the player has collided with a JumpRecharge entity.
            self.handle_jump_recharge(
                &entities,
                &mut player,
                &collision,
                &collisions,
                &jump_recharges,
            );

            // Handle everything to do with wall clinging
            // (constant velocity (for slow slide), wall jump, etc.)
            self.handle_wall_cling(
                &settings,
                &input_manager,
                &mut player,
                &mut velocity,
                (touching_horizontally_side, touching_vertically_side),
            );

            // Handle some specifics for when player is on a solid ground vs when they are in air.
            // (Resetting y velocity when on ground, etc.)
            self.handle_on_ground_and_in_air(
                &mut player,
                &mut velocity,
                (touching_horizontally_side, touching_vertically_side),
            );

            // Move left/right
            self.handle_move(
                dt,
                &settings,
                &input_handler,
                &player,
                &mut velocity,
                decr_velocity_opt,
            );

            // Regular and wall jumping
            self.handle_jump(
                &settings,
                &input_manager,
                &mut player,
                &mut velocity,
                &mut gravity_opt,
                (&audio_handler, &audio_source, &audio_output),
            );

            // Running
            self.handle_run(&input_manager, &mut player, &mut max_velocity_opt);
        }
    }
}
