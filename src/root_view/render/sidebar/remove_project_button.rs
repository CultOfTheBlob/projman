use crate::{
    config::Config,
    root_view::{
        RootView,
        render::{self, sidebar::remove_project_popup::RemoveProjectPopup},
    },
    utils,
};
use gpui::{prelude::FluentBuilder as _, *};

pub fn render(
    cx: &Context<RootView>,
    selected_project_index: Option<usize>,
) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let root_view = cx.entity();

    let disabled = selected_project_index.is_none();

    let listener = move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
        if disabled {
            return;
        }

        utils::create_popup::<RemoveProjectPopup>(&root_view, cx);
    };

    render::text_button(
        "sidebar_remove_button",
        "Remove",
        Some("󰆴"),
        &theme,
        Some(disabled),
    )
    .when(disabled, |this: Stateful<Div>| {
        this.bg(theme.background)
            .border_color(theme.background)
            .text_color(theme.text_disabled)
    })
    .on_click(listener)
}
