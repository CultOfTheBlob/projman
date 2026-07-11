use crate::app_state::AppState;
use gpui::*;

pub struct RemoveProjectPopup {
    remove_folder_checked: bool,
}

mod popup;
mod render;

impl RemoveProjectPopup {
    fn close_button_pressed(
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        AppState::set_modal_active(cx, false);
        window.remove_window();
    }

    fn confirm_button_pressed(
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut App,
    ) {
        AppState::set_modal_active(cx, false);
        window.remove_window();
    }

    fn remove_folder_checkbox_checked(
        this: &mut Self,
        _click_event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        this.remove_folder_checked = !this.remove_folder_checked;

        cx.notify();
    }
}
