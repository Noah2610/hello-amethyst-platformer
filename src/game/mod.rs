mod states;

pub mod constants {
    pub mod keys {
        use amethyst::renderer::VirtualKeyCode;

        pub const QUIT: VirtualKeyCode = VirtualKeyCode::Escape;
        pub const PAUSE: VirtualKeyCode = VirtualKeyCode::P;
    }
}

pub use states::prelude::*;
