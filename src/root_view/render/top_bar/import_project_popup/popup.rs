use std::path::PathBuf;

use crate::{
    config::Config,
    root_view::{
        popup::Popup, render::top_bar::import_project_popup::ImportProjectPopup,
    },
};
use gpui::*;
use gpui_component::input::InputState;

impl Popup for ImportProjectPopup {
    const TITLE: &'static str = "Import Project";

    const ID: &'static str = "projman.popup.importproject";

    const WIDTH_FRACTION: f32 = 0.20;

    const HEIGHT_FRACTION: f32 = 0.125;

    fn create(window: &mut Window, cx: &mut App) -> Self {
        let project_path_input_state = cx.new(|cx: &mut Context<InputState>| {
            InputState::new(window, cx).placeholder("...")
        });

        let projects_dir = &cx.global::<Config>().general.projects_dir;

        Self {
            focus_handle: cx.focus_handle(),

            project_path_input_state,

            projects_directory: PathBuf::from(projects_dir),
        }
    }
}
