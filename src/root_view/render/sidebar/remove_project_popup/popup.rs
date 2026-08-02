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

    fn create(root_view: &Entity<RootView>, _window: &mut Window, cx: &mut App) -> Self {
        let selected_project_index = root_view
            .read(cx)
            .selected_project_index
            .unwrap_or_else(|| unreachable!());

        Self {
            selected_project_index,

            remove_folder_checked: cx.global::<Config>().general.delete_project_folder,
        }
    }
}
