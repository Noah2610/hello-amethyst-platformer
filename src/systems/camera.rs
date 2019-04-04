use amethyst::ecs::world::Index;

use super::system_prelude::*;
use crate::geo::{CollisionGrid, CollisionRect, Vector};

// TODO: Move constants into settings.ron file
const BASE_SPEED: (f32, f32) = (250.0, 250.0);
const MOVE_IN_PADDING: f32 = 10.0;

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, InnerSize>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            cameras,
            players,
            sizes,
            inner_sizes,
            mut transforms,
            mut velocities,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        let player_data_opt =
            (&entities, &players, &transforms, (&sizes).maybe())
                .join()
                .next()
                .map(|(entity, player, transform, size_opt)| {
                    let translation = transform.translation();
                    (
                        entity.id(),
                        (translation.x, translation.y),
                        if let Some(size) = size_opt {
                            (size.w, size.h)
                        } else {
                            (1.0, 1.0)
                        },
                    )
                });

        if let Some((player_id, player_pos, player_size)) = player_data_opt {
            for (
                entity,
                camera,
                transform,
                size,
                inner_size_opt,
                velocity_opt,
            ) in (
                &entities,
                &cameras,
                &mut transforms,
                &sizes,
                inner_sizes.maybe(),
                (&mut velocities).maybe(),
            )
                .join()
            {
                let center =
                    (player_pos.0 - size.w * 0.5, player_pos.1 - size.h * 0.5);
                let camera_pos = transform.translation();
                let camera_id = entity.id();
                let camera_center =
                    (camera_pos.x + size.w * 0.5, camera_pos.y + size.h * 0.5);

                if let Some(inner_size) = inner_size_opt {
                    let player_rect = CollisionRect::<()>::new(
                        player_id, player_pos, // Some(player_size),
                        None,
                    );
                    let camera_rects = CameraCollisionRects::from((
                        camera_id,
                        camera_center,
                        (size.w, size.h),
                        (inner_size.0.w, inner_size.0.h),
                    ));

                    let mut colliding_any = false;

                    // Vertical rects (top/bottom)
                    if CollisionGrid::<()>::do_rects_collide(
                        &player_rect,
                        &camera_rects.top.0,
                    ) {
                        colliding_any = true;
                        transform.set_y(center.1 - inner_size.0.h * 0.5);
                    } else if CollisionGrid::<()>::do_rects_collide(
                        &player_rect,
                        &camera_rects.bottom.0,
                    ) {
                        colliding_any = true;
                        transform.set_y(center.1 + inner_size.0.h * 0.5);
                    }
                    // Horizontal rects (left/right)
                    if CollisionGrid::<()>::do_rects_collide(
                        &player_rect,
                        &camera_rects.left.0,
                    ) {
                        colliding_any = true;
                        transform.set_x(center.0 + inner_size.0.w * 0.5);
                    } else if CollisionGrid::<()>::do_rects_collide(
                        &player_rect,
                        &camera_rects.right.0,
                    ) {
                        colliding_any = true;
                        transform.set_x(center.0 - inner_size.0.w * 0.5);
                    }

                    // When not in collision with outer camera rects,
                    // slowly position camera on player.
                    if let Some(velocity) = velocity_opt {
                        if !colliding_any {
                            let dist = (
                                (player_pos.0 - camera_center.0).abs(),
                                (player_pos.1 - camera_center.1).abs(),
                            );
                            if dist.0 <= MOVE_IN_PADDING {
                                velocity.x = 0.0;
                            } else if player_pos.0 > camera_center.0 {
                                velocity.x = BASE_SPEED.0 * dist.0 * dt;
                            } else if player_pos.0 < camera_center.0 {
                                velocity.x = -BASE_SPEED.0 * dist.0 * dt;
                            }
                            if dist.1 <= MOVE_IN_PADDING {
                                velocity.y = 0.0;
                            } else if player_pos.1 > camera_center.1 {
                                velocity.y = BASE_SPEED.1 * dist.1 * dt;
                            } else if player_pos.1 < camera_center.1 {
                                velocity.y = -BASE_SPEED.1 * dist.1 * dt;
                            }
                        }
                    }
                } else {
                    transform.set_x(center.0);
                    transform.set_y(center.1);
                }
            }
        }
    }
}

struct CameraCollisionRects {
    pub top:    (CollisionRect<()>, f32),
    pub bottom: (CollisionRect<()>, f32),
    pub left:   (CollisionRect<()>, f32),
    pub right:  (CollisionRect<()>, f32),
}

impl From<(Index, Vector, Vector, Vector)> for CameraCollisionRects {
    fn from(
        (id, pos, size, inner_size): (Index, Vector, Vector, Vector),
    ) -> Self {
        let size_x = ((size.0 - inner_size.0) * 0.5, size.1);
        let size_y = (size.0, (size.1 - inner_size.1) * 0.5);
        CameraCollisionRects {
            top:    (
                CollisionRect::new(
                    id,
                    (pos.0, pos.1 + size.1 * 0.5 - size_y.1 * 0.5),
                    Some(size_y),
                ),
                pos.1,
            ),
            bottom: (
                CollisionRect::new(
                    id,
                    (pos.0, pos.1 - size.1 * 0.5 + size_y.1 * 0.5),
                    Some(size_y),
                ),
                pos.1,
            ),
            left:   (
                CollisionRect::new(
                    id,
                    (pos.0 - size.0 * 0.5 + size_x.0 * 0.5, pos.1),
                    Some(size_x),
                ),
                pos.0,
            ),
            right:  (
                CollisionRect::new(
                    id,
                    (pos.0 + size.0 * 0.5 - size_x.0 * 0.5, pos.1),
                    Some(size_x),
                ),
                pos.0,
            ),
        }
    }
}
