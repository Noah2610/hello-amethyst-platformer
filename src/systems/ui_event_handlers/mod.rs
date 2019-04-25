mod pause;

pub mod prelude {
    pub use super::UIPauseSystem;
}

mod system_prelude {
    pub use amethyst::shrev::{EventChannel, ReaderId};
    pub use amethyst::ui::UiEvent;

    pub use super::super::system_prelude::*;
}

pub use pause::PauseSystem as UIPauseSystem;
