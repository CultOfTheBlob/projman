use crate::root_view::{
    popup::Popup, render::sidebar::update_project_popup::UpdateProjectPopup,
};
use gpui::*;

impl Popup for UpdateProjectPopup {
    const TITLE: &'static str = "Update Project";

    const ID: &'static str = "projman.popup.updateproject";

    const WIDTH_FRACTION: f32 = 0.15;

    const HEIGHT_FRACTION: f32 = 0.10;

    fn create(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}
