use amethyst::ecs::world::Index;

use super::system_prelude::*;
use crate::geo::prelude::*;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Collision>,
    );

    fn run(
        &mut self,
        (entities, transforms, sizes, mut collisions): Self::SystemData,
    ) {
        // let collision_grid =
        //     CollisionGrid::from(
        //         (&entities, &transforms, sizes.maybe(), &mut collisions)
        //             .join()
        //             .map(|(entity, transform, size_opt, collision)| {
        //                 let pos = transform.translation();
        //                 (
        //                     entity.id(),
        //                     (pos.x, pos.y),
        //                     size_opt.map(|size| (size.w, size.h)),
        //                     Some(collision),
        //                 )
        //             })
        //             .collect::<Vec<(
        //                 Index,
        //                 Vector,
        //                 Option<Vector>,
        //                 Option<&mut Collision>,
        //             )>>(),
        //     );

        let collision_grid = CollisionGrid::new(
            (&entities, &transforms, sizes.maybe(), &mut collisions)
                .join()
                .map(|(entity, transform, size_opt, collision)| {
                    let pos = transform.translation();
                    CollisionRect::with_custom(
                        entity.id(),
                        (pos.x, pos.y),
                        size_opt.map(|size| (size.w, size.h)),
                        None,
                    )
                })
                .collect::<Vec<CollisionRect<()>>>(),
        );

        for (entity, collision) in (&entities, &mut collisions).join() {
            let colliding = collision_grid.colliding_with_id(entity.id());
            for other_rect in colliding {
                collision.set_collision_with(other_rect.id);
            }

            collision.update();
        }

        // for target_rect in collision_grid.rects.iter() {
        //     if let Some((target_entity, target_collision)) = &target_rect.custom
        //     {
        //         let colliding = collision_grid.colliding_with(target_rect);
        //         for other_rect in colliding {
        //             if let Some((other_entity, other_collision)) =
        //                 &target_rect.custom
        //             {
        //                 target_collision
        //                     .set_collision_with(other_entity, Side::Left)
        //             }
        //         }
        //     }
        // }
    }
}
