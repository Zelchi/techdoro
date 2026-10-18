use super::{style::*, view::View};
mod reorder;
use gpui_kit::{
    component::{
        Icon, IconName, Sizable, button::ButtonVariants, checkbox::Checkbox, input::Input,
    },
    prelude::FluentBuilder,
    *,
};

pub(super) struct TaskDrag {
    id: u64,
    pointer_y: Pixels,
    grab_offset: Pixels,
    height: Pixels,
    heights: Vec<(u64, Pixels)>,
}

impl TaskDrag {
    fn height_for(&self, id: u64) -> Pixels {
        self.heights
            .iter()
            .find(|(task, _)| *task == id)
            .map(|(_, height)| *height)
            .unwrap_or(self.height)
    }

    fn top(&self, bounds: Bounds<Pixels>) -> Pixels {
        (self.pointer_y - self.grab_offset).clamp(
            bounds.top(),
            (bounds.bottom() - self.height).max(bounds.top()),
        )
    }
}

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

    fn task_row(
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
                        self.task_drag.as_ref().is_some_and(|drag| drag.id == id),
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
    pub(super) fn start_task_drag(&mut self, id: u64, pointer_y: Pixels, cx: &mut Context<Self>) {
        let tasks = &self.model.read(cx).saved.tasks;
        let Some(index) = tasks.iter().position(|task| task.id == id) else {
            return;
        };
        let Some(bounds) = self.task_scroll.bounds_for_item(index) else {
            return;
        };
        let heights = tasks
            .iter()
            .enumerate()
            .filter_map(|(index, task)| {
                self.task_scroll
                    .bounds_for_item(index)
                    .map(|bounds| (task.id, bounds.size.height))
            })
            .collect();
        self.task_drag = Some(TaskDrag {
            id,
            pointer_y,
            grab_offset: pointer_y - bounds.top() - self.task_scroll.offset().y,
            height: bounds.size.height,
            heights,
        });
        self.model.read(cx).click();
        cx.notify();
    }

    pub(super) fn finish_task_drag(&mut self, cx: &mut Context<Self>) {
        self.reorder_dragged_task(cx);
        if self.task_drag.take().is_some() {
            self.model.update(cx, |model, _| model.persist());
            cx.notify();
        }
    }

    fn move_task_drag(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let Some(drag) = self.task_drag.as_mut() else {
            return;
        };
        drag.pointer_y = event.position.y;
        if !event.dragging() {
            self.finish_task_drag(cx);
            return;
        }
        let bounds = self.task_scroll.bounds();
        let mut offset = self.task_scroll.offset();
        if event.position.y < bounds.top() + px(24.) {
            offset.y = (offset.y + px(12.)).min(px(0.));
        } else if event.position.y > bounds.bottom() - px(24.) {
            offset.y = (offset.y - px(12.)).max(-self.task_scroll.max_offset().y);
        }
        self.task_scroll.set_offset(offset);
        self.reorder_dragged_task(cx);
        cx.notify();
    }

    fn reorder_dragged_task(&mut self, cx: &mut Context<Self>) {
        let Some(drag) = self.task_drag.as_ref() else {
            return;
        };
        let bounds = self.task_scroll.bounds();
        let top = drag.top(bounds) - bounds.top() - self.task_scroll.offset().y;
        let id = drag.id;
        let tasks = &self.model.read(cx).saved.tasks;
        let Some(source) = tasks.iter().position(|task| task.id == id) else {
            return;
        };
        let target_index = reorder::insertion_index(
            f32::from(top),
            tasks
                .iter()
                .filter(|task| task.id != id)
                .map(|task| f32::from(drag.height_for(task.id))),
            SPACING,
        );
        if source != target_index {
            let target = tasks[target_index].id;
            self.model.update(cx, |model, cx| {
                model.saved.reorder_task(id, target);
                cx.notify();
            });
        }
    }

    pub(super) fn task_list(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let tasks = self.model.read(cx).saved.tasks.clone();
        let mut top = self.task_scroll.bounds().top() + self.task_scroll.offset().y;
        let rows = tasks
            .into_iter()
            .map(|task| {
                let id = task.id;
                let row = self.task_row(task, cx);
                let row = if let Some(drag) = &self.task_drag {
                    let offset = if drag.id == id {
                        drag.top(self.task_scroll.bounds()) - top
                    } else {
                        px(0.)
                    };
                    top += drag.height_for(id) + px(SPACING);
                    deferred(
                        row.relative()
                            .top(offset)
                            .when(drag.id == id, |row| row.cursor_grabbing()),
                    )
                    .with_priority(usize::from(drag.id == id))
                    .into_any_element()
                } else {
                    row.into_any_element()
                };
                div()
                    .id(("task-slot", id))
                    .w_full()
                    .flex_shrink_0()
                    .child(row)
            })
            .collect::<Vec<_>>();
        let view = cx.entity().downgrade();
        div()
            .id("task-list")
            .relative()
            .flex()
            .flex_col()
            .gap(px(SPACING))
            .flex_1()
            .min_h_0()
            .overflow_y_scroll()
            .track_scroll(&self.task_scroll)
            .when(self.task_drag.is_some(), |list| list.cursor_grabbing())
            .children(rows)
            .child(
                canvas(
                    |_, _, _| (),
                    move |_, _, window, _| {
                        let moving_view = view.clone();
                        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _, cx| {
                            if phase == DispatchPhase::Capture {
                                let _ = moving_view.update(cx, |this, cx| {
                                    if this.task_drag.is_some() {
                                        this.move_task_drag(event, cx);
                                    }
                                });
                            }
                        });
                        let releasing_view = view.clone();
                        window.on_mouse_event(move |event: &MouseUpEvent, phase, _, cx| {
                            if phase == DispatchPhase::Capture && event.button == MouseButton::Left
                            {
                                let _ = releasing_view.update(cx, |this, cx| {
                                    if let Some(drag) = this.task_drag.as_mut() {
                                        drag.pointer_y = event.position.y;
                                        this.finish_task_drag(cx);
                                        cx.stop_propagation();
                                    }
                                });
                            }
                        });
                    },
                )
                .absolute()
                .size_0(),
            )
    }
}
