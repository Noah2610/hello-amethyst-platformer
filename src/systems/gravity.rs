use super::system_prelude::*;

pub struct GravitySystem;

impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
    );

    fn run(
        &mut self,
        (time, gravities, mut velocities, mut decr_velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (gravity, velocity, mut decr_velocity) in
            (&gravities, &mut velocities, (&mut decr_velocities).maybe()).join()
        {
            if gravity.x != 0.0 {
                velocity.x += gravity.x * dt;
                decr_velocity
                    .as_mut()
                    .map(|decr| decr.should_decrease_x = false);
            }
            if gravity.y != 0.0 {
                velocity.y += gravity.y * dt;
                decr_velocity
                    .as_mut()
                    .map(|decr| decr.should_decrease_y = false);
            }
        }
    }
}
