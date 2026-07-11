use crate::theme::Theme;
use gpui::*;

pub fn render(repo_name: String, theme: &Theme) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .p_1()
        .child(
            div()
                .text_color(theme.accent)
                .font_weight(FontWeight::MEDIUM)
                .child("Repo:"),
        )
        .child(
            div()
                .cursor_pointer()
                .text_color(theme.text)
                .child(repo_name),
        )
}
