use crate::{
    config::Config,
    root_view::{
        RootView, popup::Popup, render::sidebar::remove_project_popup::RemoveProjectPopup,
    },
};
use gpui::*;

impl Popup for RemoveProjectPopup {
    const TITLE: &'static str = "Remove Project";

    const ID: &'static str = "projman.popup.removeproject";

    const WIDTH_FRACTION: f32 = 0.20;

    const HEIGHT_FRACTION: f32 = 0.125;

    fn create(_root_view: &Entity<RootView>, _window: &mut Window, cx: &mut App) -> Self {
        Self {
            remove_folder_checked: cx.global::<Config>().general.delete_project_folder,
        }
    }
}
