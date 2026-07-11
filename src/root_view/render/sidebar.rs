use crate::{app_state::GlobalAppState, config::Config, root_view::RootView};
use gpui::*;

mod edit_project_button;
mod edit_project_popup;
mod open_project_button;
mod project_info;
mod remove_project_button;
mod remove_project_popup;
mod update_project_button;
mod update_project_popup;

pub fn render(cx: &Context<RootView>, selected_project_index: Option<usize>) -> Div {
    let app_state = cx.global::<GlobalAppState>().0.clone();
    let theme = cx.global::<Config>().theme.theme.get_theme();

    let divider_line = div()
        .w_full()
        .h(px(2.0))
        .bg(theme.border)
        .rounded_full()
        .mb_4();

    let open_project_button = open_project_button::render(cx, selected_project_index);

    let edit_project_button = edit_project_button::render(cx);

    let update_project_button = update_project_button::render(cx);

    let remove_project_button = remove_project_button::render(cx);

    let project = selected_project_index.map(|index| &app_state.projects[index]);
    let project_info = project_info::render(cx, project);

    div()
        .w(px(640.0))
        .h_full()
        .bg(theme.background_weak)
        .border_color(theme.border)
        .flex()
        .flex_col()
        .p_4()
        .text_color(theme.text_muted)
        .child(divider_line)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_4()
                .w_full()
                .child(open_project_button)
                .child(edit_project_button)
                .child(update_project_button)
                .child(remove_project_button),
        )
        .child(div().h(px(40.0)))
        .child(project_info)
}
