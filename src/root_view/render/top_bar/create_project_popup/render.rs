use crate::{
    config::Config, root_view::render::top_bar::create_project_popup::CreateProjectPopup,
};
use gpui::*;

impl Render for CreateProjectPopup {
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
            .child("CREATE")
    }
}
