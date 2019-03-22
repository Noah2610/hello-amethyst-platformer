use super::Vector;
use amethyst::ecs::world::Index;

/// A rectangular collision area with a unique entity ID
pub struct CollisionRect {
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

impl From<(Index, Vector, Option<Vector>)> for CollisionRect {
    fn from((id, pos, size): (Index, Vector, Option<Vector>)) -> Self {
        Self::new(id, pos, size)
    }
}

/// A collection of `CollisionRect`, can perform collision detection
pub struct CollisionGrid {
    rects: Vec<CollisionRect>,
}

impl CollisionGrid {
    pub fn new(rects: Vec<CollisionRect>) -> Self {
        Self { rects }
    }

    //     pub fn new<'a>(
    //         entities: &Entities<'a>,
    //         solids: &ReadStorage<'a, Solid>,
    //         sizes: &ReadStorage<'a, Size>,
    //         transforms: &WriteStorage<'a, Transform>,
    //     ) -> Self {
    //         let mut rects = Vec::new();

    //         for (entity, transform, size_opt, _) in
    //             (entities, transforms, sizes.maybe(), solids).join()
    //         {
    //             rects.push(CollisionRect::from((&entity, transform, size_opt)));
    //         }

    //         Self { rects }
    //     }

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

impl From<Vec<(Index, Vector, Option<Vector>)>> for CollisionGrid {
    fn from(data: Vec<(Index, Vector, Option<Vector>)>) -> Self {
        Self::new(
            data.iter()
                .map(|&rect_data| CollisionRect::from(rect_data))
                .collect(),
        )
    }
}
