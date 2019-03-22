use amethyst::ecs::world::Index;

use super::super::Vector;
use super::CollisionRect;

/// A collection of `CollisionRect`, can perform collision detection
pub struct CollisionGrid<T> {
    pub rects: Vec<CollisionRect<T>>,
}

impl<T> CollisionGrid<T> {
    pub fn new(rects: Vec<CollisionRect<T>>) -> Self {
        Self { rects }
    }

    pub fn rect_by_id(&self, id: Index) -> Option<&CollisionRect<T>> {
        self.rects.iter().find(|rect| id == rect.id)
    }

    pub fn collides_any(&self, target_rect: &CollisionRect<T>) -> bool {
        self.rects
            .iter()
            .any(|rect| Self::do_rects_collide(&target_rect, rect))
    }

    pub fn colliding_with(
        &self,
        target_rect: &CollisionRect<T>,
    ) -> Vec<&CollisionRect<T>> {
        self.rects
            .iter()
            .filter(|rect| Self::do_rects_collide(&target_rect, rect))
            .collect()
    }

    // pub fn colliding_with_mut(
    //     &mut self,
    //     target_rect: &CollisionRect<T>,
    // ) -> Vec<&mut CollisionRect<T>> {
    //     self.rects
    //         .iter_mut()
    //         .filter(|rect| Self::do_rects_collide(&target_rect, rect))
    //         .collect()
    // }

    pub fn colliding_with_id(&self, id: Index) -> Vec<&CollisionRect<T>> {
        if let Some(target_rect) = self.rect_by_id(id) {
            self.colliding_with(target_rect)
        } else {
            Vec::new()
        }
    }

    // pub fn colliding_with_id_mut(
    //     &mut self,
    //     id: Index,
    // ) -> Vec<&mut CollisionRect<T>> {
    //     if let Some(target_rect) = self.rects.iter().find(|rect| id == rect.id)
    //     {
    //         self.colliding_with_mut(target_rect)
    //     } else {
    //         Vec::new()
    //     }
    // }

    /// Returns a Vec with groups of colliding CollisionRects
    // pub fn collision_groups(
    //     &mut self,
    // ) -> Vec<(&mut CollisionRect<T>, Vec<&mut CollisionRect<T>>)> {
    //     let mut id_groups: Vec<(Index, Vec<Index>)> = Vec::new();

    //     unimplemented!()

    //     // self.rects
    //     //     .iter()
    //     //     .map(|rect| rect.id)
    //     //     .filter_map(|rect_id| {
    //     //         let colliding = self.colliding_with_id(rect_id);
    //     //         if colliding.is_empty() {
    //     //             None
    //     //         } else {
    //     //             Some((rect_id, colliding))
    //     //         }
    //     //     })
    //     // .map(|entry|
    //     //     ( self.rects.iter_mut().find(|rect| entry.0 == rect.id).unwrap(), entry.1 )
    //     // )
    //     //     .collect::<Vec<(&mut CollisionRect<T>, Vec<&mut CollisionRect<T>>)>>()
    // }

    #[rustfmt::skip]
    fn do_rects_collide(
        rect_one: &CollisionRect<T>,
        rect_two: &CollisionRect<T>,
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

impl<T> From<Vec<(Index, Vector, Option<Vector>)>> for CollisionGrid<T> {
    fn from(data: Vec<(Index, Vector, Option<Vector>)>) -> Self {
        Self::new(
            data.iter()
                .map(|&rect_data| CollisionRect::from(rect_data))
                .collect(),
        )
    }
}

// impl<T> From<Vec<(Index, Vector, Option<Vector>, Option<T>)>>
//     for CollisionGrid<T>
// {
//     fn from(data: Vec<(Index, Vector, Option<Vector>, Option<T>)>) -> Self {
//         Self::new(
//             data.iter()
//                 .map(|&rect_data| CollisionRect::from(rect_data))
//                 .collect(),
//         )
//     }
// }

// impl<T> Into<Iter<T>> for CollisionGrid<T> {
//     fn into(self) -> Iter<T> {
//         Iter {
//             index: 0,
//             grid:  self,
//         }
//     }
// }

// struct Iter<T> {
//     index: usize,
//     grid:  CollisionGrid<T>,
// }

// impl<T> Iterator for Iter<T> {
//     type Item = CollisionRect<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.grid.rects.first()
//     }
// }
