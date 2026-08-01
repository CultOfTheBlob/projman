use crate::{
    app_state::GlobalAppState,
    config::Config,
    project::{Project, valid_project::ValidProject},
    root_view::{RootView, render},
    utils::{self, LogType},
};
use gpui::{prelude::FluentBuilder, *};

pub fn render(
    cx: &Context<RootView>,
    selected_project_index: Option<usize>,
) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let app_state = cx.global::<GlobalAppState>().0.clone();

    let button =
        render::text_button("sidebar_open_button", "Open", Some(""), &theme, Some(true))
            .bg(theme.background)
            .border_color(theme.background)
            .text_color(theme.text_disabled);

    let project = match selected_project_index {
        Some(index) => app_state.projects[index].clone(),
        None => return button,
    };

    let disabled = matches!(project, ValidProject::Nonexistant(_));

    let button = render::text_button(
        "sidebar_open_button",
        "Open",
        Some(""),
        &theme,
        Some(disabled),
    );

    let listener = move |_: &ClickEvent, _: &mut Window, _: &mut App| {
        let ValidProject::Existant(ref project) = project else {
            return;
        };

        if let Err(err) = Project::run(project, &app_state) {
            utils::log(&err.to_string(), LogType::Error);
        }
    };

    button
        .when(disabled, |this: Stateful<Div>| {
            this.bg(theme.background)
                .border_color(theme.background)
                .text_color(theme.text_disabled)
        })
        .on_click(listener)
}
