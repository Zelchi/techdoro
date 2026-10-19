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

#[derive(Clone, Copy)]
enum SettingField {
    Volume,
    FocusMinutes,
    ShortBreakMinutes,
    LongBreakMinutes,
    CyclesBeforeLongBreak,
}

impl SettingField {
    const ALL: [Self; 5] = [
        Self::Volume,
        Self::FocusMinutes,
        Self::ShortBreakMinutes,
        Self::LongBreakMinutes,
        Self::CyclesBeforeLongBreak,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Volume => "Volume",
            Self::FocusMinutes => "Focus",
            Self::ShortBreakMinutes => "Short Break",
            Self::LongBreakMinutes => "Long Break",
            Self::CyclesBeforeLongBreak => "Cycles",
        }
    }

    const fn value(self, settings: &Settings) -> u32 {
        match self {
            Self::Volume => settings.volume,
            Self::FocusMinutes => settings.focus_minutes,
            Self::ShortBreakMinutes => settings.short_break_minutes,
            Self::LongBreakMinutes => settings.long_break_minutes,
            Self::CyclesBeforeLongBreak => settings.cycles_before_long_break,
        }
    }

    fn set(self, settings: &mut Settings, value: u32) {
        match self {
            Self::Volume => settings.volume = value,
            Self::FocusMinutes => settings.focus_minutes = value,
            Self::ShortBreakMinutes => settings.short_break_minutes = value,
            Self::LongBreakMinutes => settings.long_break_minutes = value,
            Self::CyclesBeforeLongBreak => settings.cycles_before_long_break = value,
        }
    }
}

pub(super) const INPUT_COUNT: usize = SettingField::ALL.len();

pub(super) fn create_inputs(
    model: &Entity<Model>,
    window: &mut Window,
    cx: &mut Context<View>,
) -> ([Entity<InputState>; INPUT_COUNT], Vec<Subscription>) {
    let settings = model.read(cx).saved.settings.clone();
    let inputs = SettingField::ALL.map(|field| {
        let value = field.value(&settings);
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
            .children(
                SettingField::ALL
                    .into_iter()
                    .enumerate()
                    .map(|(index, field)| {
                        div()
                            .h(px(24.))
                            .flex_shrink_0()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap(px(SPACING_SMALL))
                            .child(field.label())
                            .child(
                                Input::new(&self.settings_inputs[index])
                                    .aria_label(field.label())
                                    .small()
                                    .w(px(88.))
                                    .text_align(TextAlign::Center)
                                    .focus_bordered(false),
                            )
                    }),
            )
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
        let mut settings = previous.clone();
        for (field, input) in SettingField::ALL.into_iter().zip(&self.settings_inputs) {
            let value = input
                .read(cx)
                .value()
                .parse::<u32>()
                .unwrap_or_else(|_| field.value(&previous));
            field.set(&mut settings, value);
        }
        settings.normalize();
        if settings != previous {
            self.model.update(cx, |model, cx| {
                model.saved.settings = settings.clone();
                model.settings_changed(cx);
            });
        }
        for (field, input) in SettingField::ALL.into_iter().zip(&self.settings_inputs) {
            let text = field.value(&settings).to_string();
            if input.read(cx).value().as_str() != text {
                input.update(cx, |input, cx| input.set_value(text, window, cx));
            }
        }
    }
}
