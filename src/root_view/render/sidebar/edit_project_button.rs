use crate::{
    config::Config,
    root_view::{
        RootView,
        render::{self, sidebar::edit_project_popup::EditProjectPopup},
    },
    utils,
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();

    render::text_button("sidebar_edit_button", "Edit", Some(""), &theme, None).on_click(
        |_, _, cx: &mut App| {
            utils::create_popup::<EditProjectPopup>(cx);
        },
    )
}
