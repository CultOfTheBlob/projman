use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::valid_project::ValidProject,
    root_view::RootView,
    utils::{self, LogType},
};
use gpui::*;
use gpui_component::input::InputState;

mod existant_project;
mod nonexistant_project;

pub fn render(
    cx: &Context<RootView>,
    selected_project_index: Option<usize>,
    search_bar_state: &InputState,
) -> Div {
    let root_view = cx.entity();
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let projects = &app_state.get_filtered_projects(&search_bar_state.value());

    let project_list = div().flex().flex_col().gap_y_2p5().children(
        projects
            .iter()
            .enumerate()
            .filter_map(|(index, project)| match project {
                ValidProject::Existant(project) => {
                    let template = project
                        .template(&app_state)
                        .map_err(|err| {
                            utils::log(&err.to_string(), LogType::Error);
                        })
                        .ok()?;

                    let icon = &template.icon_path;

                    let is_selected = selected_project_index.is_some_and(|i| i == index);

                    Some(existant_project::render(
                        cx,
                        project,
                        icon,
                        is_selected,
                        index,
                    ))
                }
                ValidProject::Nonexistant(project) => {
                    let is_selected = selected_project_index.is_some_and(|i| i == index);

                    Some(nonexistant_project::render(cx, project, is_selected, index))
                }
            }),
    );

    div().flex_1().h_full().p_2().child(
        div()
            .id("project_list")
            .size_full()
            .flex()
            .flex_col()
            .overflow_y_scroll()
            .bg(theme.background)
            .rounded_lg()
            .border_1()
            .border_color(theme.border)
            .p_4()
            .text_color(theme.text)
            .child(project_list)
            .on_click(move |_, _, cx: &mut App| {
                root_view.update(
                    cx,
                    |view: &mut RootView, cx: &mut Context<RootView>| {
                        view.selected_project_index = None;

                        cx.notify();
                    },
                );
            }),
    )
}
