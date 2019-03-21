use super::system_prelude::*;
use crate::components::Axis;

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

    // TODO: Collision system
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
        let collision_grid =
            CollisionGrid::new(entities, solids, sizes, transforms);

        for (entity, velocity, size_opt, transform, _) in
            (entities, velocities, sizes.maybe(), transforms, solids).join()
        {
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
                    // Calculate new position
                    let translation = transform.translation();
                    let new_position = (
                        translation.x + if axis.is_x() { sign } else { 0.0 },
                        translation.y + if axis.is_y() { sign } else { 0.0 },
                    );
                    // Create a CollisionRect with new position
                    let collision_rect = CollisionRect::new(
                        entity.id(),
                        new_position,
                        size_opt.map(|size| (size.w, size.h)),
                    );
                    // Check for collision in newly calculated position
                    if collision_grid.collides_any(collision_rect) {
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
                let translation = transform.translation();
                let new_position = (
                    translation.x + if axis.is_x() { rem } else { 0.0 },
                    translation.y + if axis.is_y() { rem } else { 0.0 },
                );
                // Create a CollisionRect with new position
                let collision_rect = CollisionRect::new(
                    entity.id(),
                    new_position,
                    size_opt.map(|size| (size.w, size.h)),
                );
                // Check for collision in newly calculated position
                if !collision_grid.collides_any(collision_rect) {
                    // New position would NOT be in collision, apply new position
                    transform.set_x(new_position.0);
                    transform.set_y(new_position.1);
                }
            });
        }
    }
}

struct CollisionRect {
    pub id:     Index,
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
}

impl CollisionRect {
    pub fn new(
        id: Index,
        position: (f32, f32),
        size_opt: Option<(f32, f32)>,
    ) -> Self {
        if let Some(size) = size_opt {
            CollisionRect {
                id:     id,
                top:    position.1 + size.1 * 0.5,
                bottom: position.1 - size.1 * 0.5,
                left:   position.0 - size.0 * 0.5,
                right:  position.0 + size.0 * 0.5,
            }
        } else {
            CollisionRect {
                id:     id,
                top:    position.1,
                bottom: position.1,
                left:   position.0,
                right:  position.0,
            }
        }
    }
}

impl From<(&Entity, &Transform, &Size)> for CollisionRect {
    fn from((entity, transform, size): (&Entity, &Transform, &Size)) -> Self {
        let center = transform.translation();
        Self::new(entity.id(), (center.x, center.y), Some((size.w, size.h)))
    }
}

impl From<(&Entity, &Transform)> for CollisionRect {
    fn from((entity, transform): (&Entity, &Transform)) -> Self {
        let center = transform.translation();
        Self::new(entity.id(), (center.x, center.y), None)
    }
}

impl From<(&Entity, &Transform, Option<&Size>)> for CollisionRect {
    fn from(
        (entity, transform, size_opt): (&Entity, &Transform, Option<&Size>),
    ) -> Self {
        let center = transform.translation();
        Self::new(
            entity.id(),
            (center.x, center.y),
            size_opt.map(|size| (size.w, size.h)),
        )
    }
}

struct CollisionGrid {
    rects: Vec<CollisionRect>,
}

impl CollisionGrid {
    pub fn new<'a>(
        entities: &Entities<'a>,
        solids: &ReadStorage<'a, Solid>,
        sizes: &ReadStorage<'a, Size>,
        transforms: &WriteStorage<'a, Transform>,
    ) -> Self {
        let mut rects = Vec::new();

        for (entity, transform, size_opt, _) in
            (entities, transforms, sizes.maybe(), solids).join()
        {
            rects.push(CollisionRect::from((&entity, transform, size_opt)));
        }

        Self { rects }
    }

    pub fn collides_any(&self, target_rect: CollisionRect) -> bool {
        self.rects
            .iter()
            .any(|rect| Self::do_rects_collide(&target_rect, rect))
    }

    #[rustfmt::skip]
    fn do_rects_collide(
        rect_one: &CollisionRect,
        rect_two: &CollisionRect,
    ) -> bool {
        rect_one.id != rect_two.id && (
            (
                   rect_one.left >= rect_two.left
                && rect_one.left <  rect_two.right
            ) || (
                   rect_one.left  <= rect_two.left
                && rect_one.right >  rect_two.left
            )
        ) && (
            (
                   rect_one.top <= rect_two.top
                && rect_one.top >  rect_two.bottom
            ) || (
                   rect_one.top    >= rect_two.top
                && rect_one.bottom <  rect_two.top
            )
        )
    }
}
