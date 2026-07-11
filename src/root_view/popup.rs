use gpui::*;

pub trait Popup {
    const TITLE: &'static str;

    const ID: &'static str;

    const WIDTH_FRACTION: f32;

    const HEIGHT_FRACTION: f32;

    fn create(window: &mut Window, cx: &mut App) -> Self;
}
