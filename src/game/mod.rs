mod states;

pub mod constants {
    pub mod keys {
        use amethyst::renderer::VirtualKeyCode;

        pub const QUIT: VirtualKeyCode = VirtualKeyCode::Escape;
        pub const PAUSE: VirtualKeyCode = VirtualKeyCode::P;
    }

    pub const PLAYER_SIZE: (f32, f32) = (16.0, 32.0);
    pub const VIEW_DIMENSIONS: (f32, f32) = (1200.0, 800.0);
}

pub use states::prelude::*;
