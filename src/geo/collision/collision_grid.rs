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

    pub fn colliding_with_id(&self, id: Index) -> Vec<&CollisionRect<T>> {
        if let Some(target_rect) = self.rect_by_id(id) {
            self.colliding_with(target_rect)
        } else {
            Vec::new()
        }
    }

    #[rustfmt::skip]
    pub fn do_rects_collide<U, V>(
        rect_one: &CollisionRect<U>,
        rect_two: &CollisionRect<V>,
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

impl<T> From<Vec<(Index, Vector, Option<Vector>, Option<T>)>>
    for CollisionGrid<T>
where
    T: Clone + Copy,
{
    fn from(data: Vec<(Index, Vector, Option<Vector>, Option<T>)>) -> Self {
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
