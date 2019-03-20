use super::system_prelude::*;

pub struct DecreaseVelocitiesSystem;

impl<'a> System<'a> for DecreaseVelocitiesSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (time, decr_velocities, mut velocities): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (decr, velocity) in (&decr_velocities, &mut velocities).join() {
            if decr.should_decrease {
                let signx = velocity.x.signum();
                let signy = velocity.y.signum();

                // X
                if velocity.x != 0.0 {
                    velocity.x -= (decr.x * dt) * signx;
                }
                if velocity.x.signum() != signx {
                    velocity.x = 0.0;
                }

                // Y
                if velocity.y != 0.0 {
                    velocity.y -= (decr.y * dt) * signy;
                }
                if velocity.y.signum() != signy {
                    velocity.y = 0.0;
                }
            }
        }
    }
}
