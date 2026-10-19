use super::{style::*, view::View};
mod reorder;
use gpui_kit::{
    component::{
        Icon, IconName, Sizable, button::ButtonVariants, checkbox::Checkbox, input::Input,
    },
    prelude::FluentBuilder,
    *,
};
pub(super) use reorder::insertion_index;

impl View {
    pub(super) fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.input.read(cx).value().to_string();
        let editing = self.editing;
        let changed = self.model.update(cx, |model, cx| {
            let changed = if let Some(id) = editing {
                model.saved.edit_task(id, &text)
            } else {
                model.saved.add_task(&text)
            };
            model.click();
            if changed {
                model.persist();
                cx.notify();
            }
            changed
        });
        if changed {
            self.editing = None;
            self.input
                .update(cx, |input, cx| input.set_value("", window, cx));
            cx.notify();
        }
    }

    pub(super) fn edit(&mut self, id: u64, window: &mut Window, cx: &mut Context<Self>) {
        let text = self
            .model
            .read(cx)
            .saved
            .tasks
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.text.clone());
        if let Some(text) = text {
            self.model.read(cx).click();
            self.editing = Some(id);
            self.input.update(cx, |input, cx| {
                input.set_value(text, window, cx);
                input.focus(window, cx);
            });
            cx.notify();
        }
    }

    pub(super) fn cancel_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editing = None;
        self.input
            .update(cx, |input, cx| input.set_value("", window, cx));
        cx.notify();
    }

    pub(super) fn task_row(
        &self,
        task: techdoro::domain::Task,
        cx: &mut Context<Self>,
    ) -> impl IntoElement + Styled + use<> {
        let id = task.id;

        div()
            .id(("task", id))
            .flex()
            .items_center()
            .gap(px(SPACING))
            .w_full()
            .p(px(SPACING))
            .border_1()
            .border_color(rgb(BORDER))
            .bg(rgb(PANEL))
            .child(
                div()
                    .id(("drag", id))
                    .size(px(24.))
                    .flex_shrink_0()
                    .flex()
                    .items_center()
                    .justify_center()
                    .cursor_grab()
                    .text_color(rgb(0x888888))
                    .child(
                        Icon::default()
                            .data(include_bytes!("../../assets/icons/grip-vertical.svg"))
                            .size(px(16.)),
                    )
                    .when(
                        self.task_drag.as_ref().is_some_and(|drag| drag.is_for(id)),
                        |handle| handle.cursor_grabbing(),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, event: &MouseDownEvent, _, cx| {
                            this.start_task_drag(id, event.position.y, cx);
                            cx.stop_propagation();
                        }),
                    ),
            )
            .child(
                Checkbox::new(("done", id))
                    .cursor_pointer()
                    .large()
                    .checked(task.completed)
                    .on_change(cx.listener(move |this, checked: &bool, _, cx| {
                        this.model.update(cx, |model, cx| {
                            if let Some(task) = model.saved.tasks.iter_mut().find(|t| t.id == id) {
                                task.completed = *checked;
                            }
                            model.persist();
                            model.click();
                            cx.notify();
                        });
                    })),
            )
            .child(
                div()
                    .id(("edit", id))
                    .relative()
                    .top(px(2.))
                    .ml(px(SPACING_SMALL))
                    .flex_1()
                    .min_w_0()
                    .cursor_pointer()
                    .text_size(px(11.))
                    .text_color(rgb(if task.completed { 0x777777 } else { TEXT }))
                    .when(task.completed, |element| element.line_through())
                    .child(task.text)
                    .on_click(cx.listener(move |this, _, window, cx| this.edit(id, window, cx))),
            )
            .child(
                icon_button(("delete", id), IconName::Close, 24.).on_click(cx.listener(
                    move |this, _, window, cx| {
                        this.model.update(cx, |model, cx| {
                            model.saved.tasks.retain(|t| t.id != id);
                            model.persist();
                            model.click();
                            cx.notify();
                        });
                        if this.editing == Some(id) {
                            this.cancel_edit(window, cx);
                        }
                    },
                )),
            )
    }
    pub(super) fn task_input(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .h(px(32.))
            .flex_shrink_0()
            .flex()
            .items_center()
            .gap(px(SPACING))
            .child(
                Styled::h(Input::new(&self.input), px(32.))
                    .aria_label("New task")
                    .flex_1()
                    .focus_bordered(false),
            )
            .child(
                icon_button(
                    "submit",
                    if self.editing.is_some() {
                        IconName::Check
                    } else {
                        IconName::Plus
                    },
                    32.,
                )
                .primary()
                .on_click(cx.listener(|this, _, window, cx| this.submit(window, cx))),
            )
            .when(self.editing.is_some(), |element| {
                element.child(
                    icon_button("cancel", IconName::Close, 32.).on_click(cx.listener(
                        |this, _, window, cx| {
                            this.model.read(cx).click();
                            this.cancel_edit(window, cx);
                        },
                    )),
                )
            })
    }
}
