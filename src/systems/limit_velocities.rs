use super::system_prelude::*;

pub struct LimitVelocitiesSystem;

impl<'a> System<'a> for LimitVelocitiesSystem {
    type SystemData =
        (ReadStorage<'a, MaxVelocity>, WriteStorage<'a, Velocity>);

    fn run(&mut self, (max_velocities, mut velocities): Self::SystemData) {
        for (max, velocity) in (&max_velocities, &mut velocities).join() {
            if velocity.x.is_sign_positive() {
                velocity.x = velocity.x.min(max.x)
            } else if velocity.x.is_sign_negative() {
                velocity.x = velocity.x.max(-max.x)
            }
            if velocity.y.is_sign_positive() {
                velocity.y = velocity.y.min(max.y)
            } else if velocity.y.is_sign_negative() {
                velocity.y = velocity.y.max(-max.y)
            }
        }
    }
}
