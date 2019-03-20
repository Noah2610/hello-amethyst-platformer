mod states;

pub mod constants {
    pub mod keys {
        use amethyst::renderer::VirtualKeyCode;

        pub const QUIT: VirtualKeyCode = VirtualKeyCode::Escape;
        pub const PAUSE: VirtualKeyCode = VirtualKeyCode::P;
    }

    pub const VIEW_DIMENSIONS: (f32, f32) = (1200.0, 800.0);
    pub const PLAYER_SIZE: (f32, f32) = (16.0, 32.0);
    pub const PLAYER_SPEED: (f32, f32) = (500.0, 0.0);
    pub const PLAYER_JUMP_STRENGTH: f32 = 500.0;
    pub const PLAYER_MAX_VELOCITY: (f32, f32) = (200.0, 1000.0);
    pub const PLAYER_DECR_VELOCITY: (f32, f32) = (500.0, 1000.0);
}

pub use states::prelude::*;
