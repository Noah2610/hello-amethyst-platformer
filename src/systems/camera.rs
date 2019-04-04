use super::system_prelude::*;

pub struct CameraSystem;

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Transform>,
    );

    fn run(
        &mut self,
        (cameras, players, sizes, mut transforms): Self::SystemData,
    ) {
        let player_pos_opt =
            (&players, &transforms)
                .join()
                .next()
                .map(|(player, transform)| {
                    let translation = transform.translation();
                    (translation.x, translation.y)
                });

        if let Some(player_pos) = player_pos_opt {
            for (camera, transform, size) in
                (&cameras, &mut transforms, &sizes).join()
            {
                transform.set_x(player_pos.0 - size.w * 0.5);
                transform.set_y(player_pos.1 - size.h * 0.5);
            }
        }
    }
}
