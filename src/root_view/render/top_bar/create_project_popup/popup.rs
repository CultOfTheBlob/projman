use crate::root_view::{
    popup::Popup, render::top_bar::create_project_popup::CreateProjectPopup,
};
use gpui::*;

impl Popup for CreateProjectPopup {
    const TITLE: &'static str = "Create Project";

    const ID: &'static str = "projman.popup.createproject";

    const WIDTH_FRACTION: f32 = 0.15;

    const HEIGHT_FRACTION: f32 = 0.20;

    fn create(_window: &mut Window, _cx: &mut App) -> Self {
        Self {}
    }
}
