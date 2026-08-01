use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::{Existant, Project},
    root_view::RootView,
    utils::{self, LogType},
};
use gpui::*;
use std::{path::Path, sync::Arc};

pub fn render(
    cx: &Context<RootView>,
    project: &Arc<Project<Existant>>,
    icon: &Path,
    is_selected: bool,
    index: usize,
) -> Stateful<Div> {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

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
        let project = project.clone();

        move |event: &ClickEvent, _: &mut Window, cx: &mut App| {
            if event.click_count() == 2 {
                if let Err(err) = Project::run(&project, &app_state) {
                    utils::log(&err.to_string(), LogType::Error);
                    return;
                }

                root_view.update(cx, set_selected_project_index(None));
            } else {
                root_view.update(cx, set_selected_project_index(Some(index)));
            }

            cx.stop_propagation();
        }
    };

    let icon = utils::load_svg(icon, 128).map_or_else(
        || div().size_16().into_any_element(),
        |render_img| img(render_img).size_16().flex_shrink_0().into_any_element(),
    );

    let project_name = div().text_color(theme.accent).child(project.name.clone());

    let template_name = div()
        .text_color(theme.text_muted)
        .child(project.template_name.clone());

    let path = div()
        .text_color(theme.text_muted)
        .child(project.path.to_string_lossy().into_owned());

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
                .flex_col()
                .w_full()
                .gap_2()
                .child(project_name)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_4()
                        .text_color(theme.text_muted)
                        .child(template_name)
                        .child(path),
                ),
        )
}
