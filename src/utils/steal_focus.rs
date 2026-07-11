/**
Used on the parent div of windows to make them steal focus from `Input`'s.
*/
macro_rules! steal_focus {
    ($context:expr, $div:expr) => {
        $div.on_mouse_down(
            MouseButton::Left,
            $context.listener(move |this: &mut Self, _, window: &mut Window, _| {
                this.focus_handle.focus(window);
            }),
        )
    };
}

pub(crate) use steal_focus;
