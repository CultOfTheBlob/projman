use crate::app_state::AppState;
use gpui::*;
use gpui_component::input::InputState;

mod popup;
mod render;

pub struct EditProjectPopup {
    focus_handle: FocusHandle,

    project_name_input_state: Entity<InputState>,
    project_repo_input_state: Entity<InputState>,
}

impl EditProjectPopup {
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
}
