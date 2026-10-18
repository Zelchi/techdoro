use super::{style::*, view::View};
use crate::app::Model;
use gpui_kit::{
    component::{
        Sizable,
        input::{Input, InputEvent, InputState},
    },
    *,
};
use techdoro::domain::Settings;

const LABELS: [&str; 5] = ["Volume", "Focus", "Short Break", "Long Break", "Cycles"];

fn values(settings: &Settings) -> [u32; 5] {
    [
        settings.volume,
        settings.focus_minutes,
        settings.short_break_minutes,
        settings.long_break_minutes,
        settings.cycles_before_long_break,
    ]
}

pub(super) fn create_inputs(
    model: &Entity<Model>,
    window: &mut Window,
    cx: &mut Context<View>,
) -> ([Entity<InputState>; 5], Vec<Subscription>) {
    let inputs = values(&model.read(cx).saved.settings).map(|value| {
        cx.new(|cx| {
            InputState::new(window, cx)
                .default_value(value.to_string())
                .validate(|text, _| {
                    text.len() <= 4 && text.bytes().all(|byte| byte.is_ascii_digit())
                })
        })
    });
    let subscriptions = inputs
        .iter()
        .map(|input| {
            cx.subscribe_in(input, window, |this, _, event, window, cx| {
                if matches!(event, InputEvent::PressEnter { .. } | InputEvent::Blur) {
                    this.save_settings(window, cx);
                }
            })
        })
        .collect();
    (inputs, subscriptions)
}

impl View {
    pub(super) fn settings(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .size_full()
            .flex_shrink_0()
            .flex()
            .flex_col()
            .justify_center()
            .gap(px(SPACING_SMALL))
            .px(px(SPACING * 2.))
            .py(px(SPACING))
            .bg(rgb(PANEL))
            .border_1()
            .border_color(rgb(BORDER))
            .children(LABELS.into_iter().enumerate().map(|(index, label)| {
                div()
                    .h(px(24.))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap(px(SPACING_SMALL))
                    .child(label)
                    .child(
                        Input::new(&self.settings_inputs[index])
                            .aria_label(label)
                            .small()
                            .w(px(88.))
                            .text_align(TextAlign::Center)
                            .focus_bordered(false),
                    )
            }))
    }

    pub(super) fn toggle_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let previous = self.model.read(cx).saved.settings.clone();
        if self.settings_open {
            self.save_settings(window, cx);
            window.blur(cx);
        }
        if self.model.read(cx).saved.settings == previous {
            self.model.read(cx).click();
        }
        self.settings_open = !self.settings_open;
        cx.notify();
    }

    fn save_settings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let previous = self.model.read(cx).saved.settings.clone();
        let previous_values = values(&previous);
        let [
            volume,
            focus_minutes,
            short_break_minutes,
            long_break_minutes,
            cycles_before_long_break,
        ] = std::array::from_fn(|index| {
            self.settings_inputs[index]
                .read(cx)
                .value()
                .parse::<u32>()
                .unwrap_or(previous_values[index])
        });
        let mut settings = Settings {
            volume,
            focus_minutes,
            short_break_minutes,
            long_break_minutes,
            cycles_before_long_break,
        };
        settings.normalize();
        if settings != previous {
            self.model.update(cx, |model, cx| {
                model.saved.settings = settings.clone();
                model.settings_changed(cx);
            });
        }
        for (input, value) in self.settings_inputs.iter().zip(values(&settings)) {
            let text = value.to_string();
            if input.read(cx).value().as_str() != text {
                input.update(cx, |input, cx| input.set_value(text, window, cx));
            }
        }
    }
}
