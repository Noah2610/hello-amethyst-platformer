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

        for (gravity, velocity, decr_velocity) in
            (&gravities, &mut velocities, &mut decr_velocities).join()
        {
            if gravity.x != 0.0 {
                velocity.x += gravity.x * dt;
                decr_velocity.should_decrease_x = false;
            } else {
                decr_velocity.should_decrease_x = true;
            }
            if gravity.y != 0.0 {
                velocity.y += gravity.y * dt;
                decr_velocity.should_decrease_y = false;
            } else {
                decr_velocity.should_decrease_y = true;
            }
        }
    }
}
