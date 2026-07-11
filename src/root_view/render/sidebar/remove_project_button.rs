use crate::{
    config::Config,
    root_view::{
        RootView,
        render::{self, sidebar::remove_project_popup::RemoveProjectPopup},
    },
    utils,
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();

    render::text_button("sidebar_remove_button", "Remove", Some("󰆴"), &theme, None)
        .on_click(|_, _, cx: &mut App| {
            utils::create_popup::<RemoveProjectPopup>(cx);
        })
}
