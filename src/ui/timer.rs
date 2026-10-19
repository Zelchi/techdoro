use super::{style::*, view::View};
use gpui_kit::{
    component::{
        Disableable, FocusableExt, IconName,
        button::{Button, ButtonVariants},
    },
    prelude::FluentBuilder,
    *,
};
use std::time::{Duration, Instant};

fn timer_text(remaining: Duration, maximum: Duration) -> String {
    let seconds = remaining.as_secs() + u64::from(remaining.subsec_nanos() > 0);
    if maximum.as_secs() >= 3600 {
        format!(
            "{:02}:{:02}:{:02}",
            seconds / 3600,
            seconds / 60 % 60,
            seconds % 60
        )
    } else {
        format!("{:02}:{:02}", seconds / 60, seconds % 60)
    }
}

impl View {
    pub(super) fn session_controls(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let label = self.model.read(cx).timer.session.label();
        div()
            .flex()
            .items_center()
            .justify_between()
            .child(
                icon_button("next", IconName::ChevronRight, 24.).on_click(
                    cx.listener(|this, _, _, cx| this.model.update(cx, |m, cx| m.next(cx))),
                ),
            )
            .child(div().text_color(rgb(ORANGE)).child(label))
            .child(
                icon_button("settings", IconName::Settings, 24.)
                    .on_click(cx.listener(|this, _, window, cx| this.toggle_settings(window, cx))),
            )
    }
    pub(super) fn timer(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let model = self.model.read(cx);
        let remaining = model.timer.remaining(Instant::now());
        let progress =
            (remaining.as_secs_f32() / model.timer.maximum().as_secs_f32()).clamp(0., 1.);
        let running = model.timer.running();
        let can_reset = running || remaining < model.timer.maximum();
        let installing_update = model.installing_update;
        div()
            .size_full()
            .flex_shrink_0()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(SPACING))
            .py(px(SPACING))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(SPACING * 2.))
                    .child(
                        div()
                            .text_size(px(48.))
                            .line_height(relative(1.))
                            .child(timer_text(remaining, model.timer.maximum())),
                    )
                    .child(
                        div()
                            .w(px(215.))
                            .h(px(10.))
                            .flex_shrink_0()
                            .border_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(PANEL))
                            .child(div().h_full().w(relative(progress)).bg(rgb(ORANGE))),
                    ),
            )
            .child(
                div()
                    .relative()
                    .w(px(112.))
                    .h(px(40.))
                    .flex_shrink_0()
                    .mt(px(SPACING * 2.))
                    .child(
                        Button::new("toggle")
                            .focus_ring(false)
                            .when(!installing_update, |button| button.cursor_pointer())
                            .w_full()
                            .h(px(40.))
                            .rounded(px(0.))
                            .primary()
                            .accessibility_label(if running { "Pause" } else { "Start" })
                            .child(div().text_size(px(16.)).child(if running {
                                "Pause"
                            } else {
                                "Start"
                            }))
                            .disabled(installing_update)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.model.update(cx, |m, cx| m.toggle(cx))
                            })),
                    )
                    .when(can_reset, |controls| {
                        controls.child(
                            div()
                                .absolute()
                                .left_full()
                                .top_0()
                                .ml(px(SPACING))
                                .h(px(40.))
                                .flex()
                                .items_center()
                                .child(
                                    icon_button("reset", IconName::RefreshCw, 32.)
                                        .accessibility_label("Reset")
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.model.update(cx, |m, cx| m.reset(cx))
                                        })),
                                ),
                        )
                    }),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::timer_text;
    use std::time::Duration;

    #[test]
    fn clock_format_matches_session_duration_and_stays_stable_during_countdown() {
        for (remaining, maximum, expected) in [
            (1500, 1500, "25:00"),
            (3599, 3599, "59:59"),
            (3600, 3600, "01:00:00"),
            (5400, 5400, "01:30:00"),
            (3599, 5400, "00:59:59"),
            (0, 5400, "00:00:00"),
            (86400, 86400, "24:00:00"),
        ] {
            assert_eq!(
                timer_text(Duration::from_secs(remaining), Duration::from_secs(maximum)),
                expected
            );
        }
        assert_eq!(
            timer_text(Duration::from_millis(1), Duration::from_secs(5400)),
            "00:00:01"
        );
    }
}
