use dioxus::prelude::*;

use super::MaybeSignal;

/// A two-state button that can be `on` or `off`.
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"on"` | `"off"`
/// - `[data-disabled]`: Present when disabled
/// 
/// # Accessibility
/// 
/// **Keyboard Interaction**
/// 
/// - `<Space>`: Activates/Deactivates the toggle.
/// - `<Enter>`: Activates/Deactivates the toggle.
#[component]
pub fn Toggle(
    pressed: Option<bool>,
    disabled: Option<bool>,

    onchange: Option<EventHandler<bool>>,

    children: Option<Element>,

    #[props(extends = GlobalAttributes, extends = Button)]
    button_and_global_attrs: Vec<Attribute>,
) -> Element {
    let mut state = use_signal(|| pressed.unwrap_or_default());

    use_effect(use_reactive!(|pressed| {
        if let Some(pressed) = pressed {
            state.set(pressed);
        } 
    }));

    rsx! {
        button {
            r#type: "button",

            aria_pressed: state,
            "data-state": if state() { "on" } else { "off" },

            disabled: disabled,
            aria_disabled: disabled,
            "data-disabled": disabled.unwrap_or_default(),

            onclick: move |_| {
                state.toggle();
                if let Some(onchange) = onchange {
                    onchange.call(*state.read());
                }
            },

            ..button_and_global_attrs,

            {children}
        }
    }
}