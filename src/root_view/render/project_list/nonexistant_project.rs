use crate::{
    config::Config,
    project::{Nonexistant, Project},
    root_view::RootView,
};
use gpui::*;
use std::sync::Arc;

pub fn render(
    cx: &Context<RootView>,
    project: &Arc<Project<Nonexistant>>,
    is_selected: bool,
    index: usize,
) -> Stateful<Div> {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();

    let bg_color = if is_selected {
        theme.surface_strong
    } else {
        theme.background
    };

    let border_color = if is_selected {
        theme.accent
    } else {
        theme.background_weak
    };

    let set_selected_project_index = |index: Option<usize>| {
        move |view: &mut RootView, cx: &mut Context<RootView>| {
            view.selected_project_index = index;

            cx.notify();
        }
    };

    let listener = {
        move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
            root_view.update(cx, set_selected_project_index(Some(index)));

            cx.stop_propagation();
        }
    };

    let icon = div()
        .flex()
        .flex_row()
        .items_center()
        .size_16()
        .text_color(theme.error)
        .text_size(px(96.0))
        .child("");

    let title = div().text_color(theme.error).child("Missing!");

    let project_name = div()
        .text_color(theme.text_muted)
        .child(project.name.clone());

    div()
        .id(SharedString::from(format!("project_{}", project.name)))
        .flex()
        .flex_row()
        .w_full()
        .items_center()
        .p_4()
        .gap_4()
        .bg(bg_color)
        .border_1()
        .border_color(border_color)
        .rounded_lg()
        .cursor_pointer()
        .hover(|style: StyleRefinement| style.bg(theme.surface))
        .on_click(listener)
        .child(icon)
        .child(
            div()
                .flex()
                .flex_row()
                .w_full()
                .gap_4()
                .child(title)
                .child(div().text_color(theme.text_muted).child(project_name)),
        )
}
