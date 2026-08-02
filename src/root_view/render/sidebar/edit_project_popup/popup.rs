use crate::root_view::{
    RootView, popup::Popup, render::sidebar::edit_project_popup::EditProjectPopup,
};
use gpui::*;
use gpui_component::input::InputState;

impl Popup for EditProjectPopup {
    const TITLE: &'static str = "Edit Project";

    const ID: &'static str = "projman.popup.editproject";

    const WIDTH_FRACTION: f32 = 0.20;

    const HEIGHT_FRACTION: f32 = 0.20;

    fn create(_root_view: &Entity<RootView>, window: &mut Window, cx: &mut App) -> Self {
        let project_name_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("Name")
        });

        let project_repo_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("Repo")
        });

        Self {
            focus_handle: cx.focus_handle(),

            project_name_input_state,
            project_repo_input_state,
        }
    }
}
