use crate::{
    config::Config, root_view::render::sidebar::update_project_popup::UpdateProjectPopup,
};
use gpui::*;

impl Render for UpdateProjectPopup {
    fn render(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.theme.get_theme();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.surface)
            .p_4()
            .text_color(theme.text)
            .child("UPDATE")
    }
}
