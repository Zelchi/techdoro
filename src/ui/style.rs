use gpui_kit::{
    component::{FocusableExt, IconName, Sizable, button::Button},
    *,
};
use std::sync::{Arc, OnceLock};

pub(super) const BG: u32 = 0x0a0a0a;
pub(super) const PANEL: u32 = 0x121212;
pub(super) const BORDER: u32 = 0x2b2b2b;
pub(super) const ORANGE: u32 = 0xff6a00;
pub(super) const TEXT: u32 = 0xe6e6e6;
pub(super) const SPACING: f32 = 8.;
pub(super) const SPACING_SMALL: f32 = 4.;
pub(super) const SESSION_PANEL_HEIGHT: f32 = 156.;

pub(super) fn application_icon() -> Arc<Image> {
    static ICON: OnceLock<Arc<Image>> = OnceLock::new();
    ICON.get_or_init(|| {
        Arc::new(Image::from_bytes(
            ImageFormat::Png,
            include_bytes!("../../assets/icon.png").to_vec(),
        ))
    })
    .clone()
}

pub(super) fn icon_button(id: impl Into<ElementId>, icon: IconName, edge: f32) -> Button {
    Button::new(id)
        .focus_ring(false)
        .cursor_pointer()
        .small()
        .size(px(edge))
        .rounded(px(0.))
        .icon(icon)
}
