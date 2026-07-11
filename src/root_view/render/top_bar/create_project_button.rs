use crate::{
    config::Config,
    root_view::{
        RootView,
        render::{self, top_bar::create_project_popup::CreateProjectPopup},
    },
    utils,
};
use gpui::*;

pub fn render(cx: &Context<RootView>) -> Stateful<Div> {
    let theme = cx.global::<Config>().theme.theme.get_theme();

    render::text_button("top_bar_create_button", "Create", None, &theme, None).on_click(
        |_, _, cx: &mut App| {
            utils::create_popup::<CreateProjectPopup>(cx);
        },
    )
}
