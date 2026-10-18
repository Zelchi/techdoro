use super::{style::*, titlebar::titlebar};
use crate::app::{Model, Runtime};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    component::input::{InputEvent, InputState},
    *,
};

pub struct View {
    pub(super) model: Entity<Model>,
    pub(super) input: Entity<InputState>,
    pub(super) settings_inputs: [Entity<InputState>; 5],
    pub(super) editing: Option<u64>,
    pub(super) settings_open: bool,
    _subscriptions: Vec<Subscription>,
}

impl View {
    pub fn new(model: Entity<Model>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx));
        let subscription = cx.subscribe_in(&input, window, |this, _, event, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.submit(window, cx);
            }
        });
        let observe = cx.observe(&model, |_, _, cx| cx.notify());
        let (settings_inputs, mut subscriptions) =
            super::settings::create_inputs(&model, window, cx);
        subscriptions.extend([subscription, observe]);
        Self {
            model,
            input,
            settings_inputs,
            editing: None,
            settings_open: false,
            _subscriptions: subscriptions,
        }
    }
}

impl Render for View {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let error = self.model.read(cx).error.clone();
        let tray_available = cx.global::<Runtime>().tray.is_some();
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BG))
            .text_color(rgb(TEXT))
            .font_family("Press Start 2P")
            .text_size(px(10.))
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                if event.keystroke.key == "escape" {
                    if this.editing.is_some() {
                        this.cancel_edit(window, cx);
                    } else if this.settings_open {
                        this.toggle_settings(window, cx);
                    }
                }
            }))
            .child(titlebar())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h_0()
                    .p(px(SPACING))
                    .gap(px(SPACING))
                    .child(self.session_controls(cx))
                    .child(
                        div()
                            .w_full()
                            .h(px(SESSION_PANEL_HEIGHT))
                            .flex_shrink_0()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(if self.settings_open {
                                self.settings(cx).into_any_element()
                            } else {
                                self.timer(cx).into_any_element()
                            }),
                    )
                    .child(self.task_input(cx))
                    .child(self.task_list(cx)),
            )
            .when_some(error, |element, error| {
                element.child(
                    div()
                        .p(px(SPACING))
                        .text_size(px(9.))
                        .text_color(rgb(0xff7777))
                        .child(error),
                )
            })
            .when(!tray_available, |element| {
                element.child(
                    div()
                        .p(px(SPACING))
                        .text_size(px(9.))
                        .text_color(rgb(0xffcc77))
                        .child("System tray unavailable in this session."),
                )
            })
            .child(self.update_footer(cx))
    }
}
