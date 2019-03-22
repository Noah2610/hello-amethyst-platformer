#[derive(Clone, Debug, PartialEq)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn for_each<C>(mut iterate: C)
    where
        C: FnMut(Self),
    {
        iterate(Axis::X);
        iterate(Axis::Y);
    }

    pub fn is_x(&self) -> bool {
        Axis::X == *self
    }

    pub fn is_y(&self) -> bool {
        Axis::Y == *self
    }
}
