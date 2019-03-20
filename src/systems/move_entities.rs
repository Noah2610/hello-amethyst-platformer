use super::system_prelude::*;

pub struct MoveEntitiesSystem;

impl<'a> System<'a> for MoveEntitiesSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Transform>,
    );

    // TODO: Collision system
    fn run(&mut self, (time, velocities, mut transforms): Self::SystemData) {
        let dt = time.delta_seconds();
        for (velocity, transform) in (&velocities, &mut transforms).join() {
            transform.translate_x(velocity.x * dt);
            transform.translate_y(velocity.y * dt);
        }
    }
}
