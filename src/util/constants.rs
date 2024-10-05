pub const APPLICATION_VERSION: u32 = make_version(0, 1, 0);
pub const ENGINE_VERSION: u32 = make_version(0, 1, 0);
pub const API_VERSION: u32 = make_version(0, 1, 0);

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub const APPLICATION_NAME: &'static str = "NeoClausewitz.Test";
pub const ENGINE_NAME: &'static str = "NeoClausewitz";

pub const fn make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}