use gpui::*;

use crate::root_view::RootView;

pub trait Popup {
    const TITLE: &'static str;

    const ID: &'static str;

    const WIDTH_FRACTION: f32;

    const HEIGHT_FRACTION: f32;

    fn create(root_view: &Entity<RootView>, window: &mut Window, cx: &mut App) -> Self;
}
