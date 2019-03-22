use super::system_prelude::*;
use crate::geo::prelude::*;

pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, Velocity>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (entities, time, velocities, solids, sizes, mut transforms): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        self.run_without_collision(dt, &velocities, &solids, &mut transforms);

        self.run_with_collision(
            dt,
            &entities,
            &velocities,
            &solids,
            &sizes,
            &mut transforms,
        );
    }
}

impl<'a> MoveEntitiesSystem {
    fn run_without_collision(
        &self,
        dt: f32,
        velocities: &ReadStorage<'a, Velocity>,
        solids: &ReadStorage<Solid>,
        transforms: &mut WriteStorage<'a, Transform>,
    ) {
        for (velocity, transform, _) in (velocities, transforms, !solids).join()
        {
            transform.translate_x(velocity.x * dt);
            transform.translate_y(velocity.y * dt);
        }
    }

    fn run_with_collision(
        &self,
        dt: f32,
        entities: &Entities<'a>,
        velocities: &ReadStorage<'a, Velocity>,
        solids: &ReadStorage<'a, Solid>,
        sizes: &ReadStorage<'a, Size>,
        transforms: &mut WriteStorage<'a, Transform>,
    ) {
        // Generate CollisionGrid with all solid entities
        let collision_grid = CollisionGrid::<()>::from(
            (entities, &*transforms, sizes.maybe(), solids)
                .join()
                .map(|(entity, transform, size_opt, _)| {
                    let pos = transform.translation();
                    (
                        entity.id(),
                        (pos.x, pos.y),
                        size_opt.map(|size| (size.w, size.h)),
                    )
                })
                .collect::<Vec<(Index, (f32, f32), Option<(f32, f32)>)>>(),
        );

        for (entity, velocity, size_opt, transform, _) in
            (entities, velocities, sizes.maybe(), transforms, solids).join()
        {
            let entity_id = entity.id();
            Axis::for_each(|axis| {
                let vel = match axis {
                    Axis::X => velocity.x * dt,
                    Axis::Y => velocity.y * dt,
                };
                let abs = vel.abs() as usize;
                let sign = if vel != 0.0 { vel.signum() } else { 0.0 };
                let rem = vel % 1.0;

                // Try to move by one absolute unit
                for _ in 0 ..= abs {
                    let (collision_rect, new_position) =
                        new_collision_rect_and_position(
                            entity_id, transform, size_opt, &axis, sign,
                        );
                    // Check for collision in newly calculated position
                    if collision_grid.collides_any(&collision_rect) {
                        // New position would be in collision, break out of loop and don't apply
                        // new position
                        break;
                    } else {
                        // New position would NOT be in collision, apply new position
                        transform.set_x(new_position.0);
                        transform.set_y(new_position.1);
                    }
                }
                // Try to move by the floating point remainder
                // Calculate new position
                let (collision_rect, new_position) =
                    new_collision_rect_and_position(
                        entity_id, transform, size_opt, &axis, rem,
                    );
                // Check for collision in newly calculated position
                if !collision_grid.collides_any(&collision_rect) {
                    // New position would NOT be in collision, apply new position
                    transform.set_x(new_position.0);
                    transform.set_y(new_position.1);
                }
            });
        }
    }
}

fn new_collision_rect_and_position<T>(
    id: Index,
    transform: &Transform,
    size_opt: Option<&Size>,
    axis: &Axis,
    step: f32,
) -> (CollisionRect<T>, Vector) {
    // Calculate new position
    let pos = transform.translation();
    let new_position = (
        pos.x + if axis.is_x() { step } else { 0.0 },
        pos.y + if axis.is_y() { step } else { 0.0 },
    );
    // Create a CollisionRect with new position
    (
        CollisionRect::new(
            id,
            new_position,
            size_opt.map(|size| (size.w, size.h)),
        ),
        new_position,
    )
}
