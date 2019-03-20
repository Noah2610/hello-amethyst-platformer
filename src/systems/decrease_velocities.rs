use super::system_prelude::*;

pub struct DecreaseVelocitiesSystem;

impl<'a> System<'a> for DecreaseVelocitiesSystem {
    type SystemData = (
        ReadStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (decr_velocities, mut velocities): Self::SystemData) {
        for (decr, velocity) in (&decr_velocities, &mut velocities).join() {
            let signx = velocity.x.signum();
            let signy = velocity.y.signum();

            // X
            if velocity.x != 0.0 {
                velocity.x -= decr.x * signx;
            }
            if velocity.x.signum() != signx {
                velocity.x = 0.0;
            }

            // Y
            if velocity.y != 0.0 {
                velocity.y -= decr.y * signy;
            }
            if velocity.y.signum() != signy {
                velocity.y = 0.0;
            }
        }
    }
}
