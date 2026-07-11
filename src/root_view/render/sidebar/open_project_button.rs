use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::Project,
    root_view::{RootView, render},
    utils::{self, LogType},
};
use gpui::*;

pub fn render(
    cx: &Context<RootView>,
    selected_project_index: Option<usize>,
) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    selected_project_index.map_or_else(
        || render::text_button("sidebar_open_button", "Open", Some(""), &theme, None),
        |index| {
            let project = app_state.projects[index].clone();
            let app_state = app_state.clone();

            let listener = move |_: &ClickEvent, _: &mut Window, _: &mut App| {
                if let Err(err) = Project::run(&project, &app_state) {
                    utils::log(&err.to_string(), LogType::Error);
                }
            };

            render::text_button("sidebar_open_button", "Open", Some(""), &theme, None)
                .on_click(listener)
        },
    )
}
