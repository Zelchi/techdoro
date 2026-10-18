use super::{style::*, view::View};
use gpui_kit::{
    component::{Disableable, Sizable, button::Button},
    prelude::FluentBuilder,
    *,
};

impl View {
    pub(super) fn update_footer(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let model = self.model.read(cx);
        let update_available = model.update.is_some();
        let update_busy = model.update_busy;
        let running = model.timer.running();
        let update_status = if update_available && running && !update_busy {
            "Pause the timer to update.".into()
        } else {
            model.update_status.clone()
        };
        div()
            .flex_shrink_0()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(SPACING))
            .p(px(SPACING))
            .border_t_1()
            .border_color(rgb(BORDER))
            .child(
                div()
                    .flex_1()
                    .text_size(px(8.))
                    .text_color(rgb(0x888888))
                    .child(if update_status.is_empty() {
                        format!("v{}", env!("CARGO_PKG_VERSION"))
                    } else {
                        update_status
                    }),
            )
            .when(update_available, |footer| {
                footer.child(
                    Button::new("update")
                        .when(!update_busy && !running, |button| button.cursor_pointer())
                        .rounded(px(0.))
                        .small()
                        .label("Update")
                        .disabled(update_busy || running)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.model.read(cx).click();
                            this.model.update(cx, |model, cx| model.install_update(cx));
                        })),
                )
            })
    }
}
