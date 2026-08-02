use std::sync::Arc;

use crate::app_state::{AppState, GlobalAppState};
use gpui::*;

pub struct RemoveProjectPopup {
    selected_project_index: usize,

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
        view: &mut Self,
        _click_event: &ClickEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.update_global::<GlobalAppState, ()>(|app_state: &mut GlobalAppState, _| {
            let app_state = Arc::make_mut(&mut app_state.0);
            app_state.remove_project(view.selected_project_index);
        });

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
