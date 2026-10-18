use gpui_kit::{
    component::{Theme, ThemeMode},
    *,
};
use std::borrow::Cow;

pub(crate) fn init_theme(cx: &mut App) {
    if let Err(error) = cx
        .text_system()
        .add_fonts(vec![Cow::Borrowed(include_bytes!(
            "../../assets/PressStart2P-Regular.ttf"
        ))])
    {
        eprintln!("Could not load font: {error}");
    }
    Theme::change(ThemeMode::Dark, None, cx);
    let theme = Theme::global_mut(cx);
    theme.radius = px(0.);
    theme.radius_lg = px(0.);
    theme.primary = rgb(0xffa500).into();
    theme.primary_foreground = rgb(0x0a0a0a).into();
    theme.primary_hover = rgb(0xff8c00).into();
    theme.primary_active = rgb(0xe07800).into();
    theme.button_primary = theme.primary;
    theme.button_primary_foreground = theme.primary_foreground;
    theme.button_primary_hover = theme.primary_hover;
    theme.button_primary_active = theme.primary_active;
    theme.font_family = "Press Start 2P".into();
    theme.font_size = px(11.);
    theme.tokens = gpui_kit::component::ThemeTokens::from(theme.colors);
}
