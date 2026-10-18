use super::Model;
use gpui_kit::{AnyWindowHandle, Entity, Global};
use gpui_tray::Tray;

pub struct Runtime {
    pub model: Entity<Model>,
    pub window: Option<AnyWindowHandle>,
    pub window_opening: bool,
    pub tray: Option<Tray>,
}
impl Global for Runtime {}
