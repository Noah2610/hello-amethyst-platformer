use amethyst::ecs::world::Index;

use super::super::Vector;

/// A rectangular collision area with a unique entity ID
pub struct CollisionRect<T> {
    pub id:     Index,
    pub top:    f32,
    pub bottom: f32,
    pub left:   f32,
    pub right:  f32,
    pub custom: Option<T>, // Optional, custom data
}

impl<T> CollisionRect<T> {
    pub fn new(
        id: Index,
        position: (f32, f32),
        size_opt: Option<(f32, f32)>,
    ) -> Self {
        Self::with_custom(id, position, size_opt, None)
    }

    pub fn with_custom(
        id: Index,
        position: (f32, f32),
        size_opt: Option<(f32, f32)>,
        custom: Option<T>,
    ) -> Self {
        if let Some(size) = size_opt {
            CollisionRect {
                id:     id,
                top:    position.1 + size.1 * 0.5,
                bottom: position.1 - size.1 * 0.5,
                left:   position.0 - size.0 * 0.5,
                right:  position.0 + size.0 * 0.5,
                custom: custom,
            }
        } else {
            CollisionRect {
                id:     id,
                top:    position.1,
                bottom: position.1,
                left:   position.0,
                right:  position.0,
                custom: custom,
            }
        }
    }
}

impl<T> From<(Index, Vector, Option<Vector>)> for CollisionRect<T> {
    fn from((id, pos, size): (Index, Vector, Option<Vector>)) -> Self {
        Self::new(id, pos, size)
    }
}

impl<T> From<(Index, Vector, Option<Vector>, Option<T>)> for CollisionRect<T> {
    fn from(
        (id, pos, size, custom): (Index, Vector, Option<Vector>, Option<T>),
    ) -> Self {
        Self::with_custom(id, pos, size, custom)
    }
}
