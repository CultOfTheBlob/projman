use crate::{
    config::Config,
    root_view::{
        RootView,
        render::{self, sidebar::update_project_popup::UpdateProjectPopup},
    },
    utils,
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();
    let root_view = cx.entity();

    render::text_button("sidebar_update_button", "Update", Some(""), &theme, None)
        .on_click(move |_, _, cx: &mut App| {
            utils::create_popup::<UpdateProjectPopup>(&root_view, cx);
        })
}
