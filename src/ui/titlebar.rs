use super::style::*;
use crate::app::{Hide, Runtime};
use gpui_kit::{component::IconName, *};

pub(super) fn titlebar() -> impl IntoElement {
    div()
        .flex_shrink_0()
        .flex()
        .items_center()
        .justify_between()
        .p(px(SPACING))
        .bg(rgb(PANEL))
        .border_b_1()
        .border_color(rgb(BORDER))
        .cursor_grab()
        .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
        .child(
            div()
                .size(px(24.))
                .flex()
                .items_center()
                .justify_center()
                .border_1()
                .border_color(rgb(BORDER))
                .child(img(application_icon()).size(px(16.))),
        )
        .child(div().id("title").flex_1().text_center().child("Techdoro"))
        .child(
            div()
                .size(px(24.))
                .cursor_default()
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .child(
                    icon_button("close", IconName::Close, 24.).on_click(|_, window, cx| {
                        cx.global::<Runtime>().model.read(cx).click();
                        window.dispatch_action(Box::new(Hide), cx);
                    }),
                ),
        )
}
