/**
Used when adding a new `Input` to a div, allows the `Input` to be focusable even when the parent div is stealing focus.
*/
macro_rules! input {
    ($input:expr) => {
        div()
            .child($input)
            .on_mouse_down(MouseButton::Left, move |_, _, cx: &mut App| {
                cx.stop_propagation();
            })
    };
}

pub(crate) use input;
