use std::sync::Arc;

use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::{Project, valid_project::ValidProject},
    root_view::{
        RootView,
        render::{self, top_bar::create_project_popup::CreateProjectPopup},
    },
    utils,
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let config = cx.global::<Config>();
    let root_view = cx.entity();

    let project = ValidProject::Existant(Arc::new(Project::new(config)));

    render::text_button("top_bar_create_button", "Create", None, &theme, None).on_click(
        move |_, _, cx: &mut App| {
            let project = project.clone();

            cx.update_global::<GlobalAppState, ()>(
                |app_state: &mut GlobalAppState, _| {
                    let _ = Arc::make_mut(&mut app_state.0).add_project(project);
                },
            );

            utils::create_popup::<CreateProjectPopup>(&root_view, cx);
        },
    )
}
