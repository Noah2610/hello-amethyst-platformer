#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Side {
    // `Inner` isn't actually a side, but it makes life easier having this here
    Inner,
    Top,
    Bottom,
    Left,
    Right,
}
