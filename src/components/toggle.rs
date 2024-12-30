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
    #[props(into, default = MaybeSignal::new(false))]
    pressed: MaybeSignal<bool>,
    disabled: Option<bool>,

    onchange: Option<EventHandler<bool>>,

    children: Option<Element>,

    #[props(extends = GlobalAttributes, extends = Button)]
    button_and_global_attrs: Vec<Attribute>,
) -> Element {
    let mut pressed = pressed.as_signal();

    rsx! {
        button {
            r#type: "button",

            aria_pressed: pressed,
            "data-state": if pressed() { "on" } else { "off" },

            disabled: disabled,
            aria_disabled: disabled,
            "data-disabled": disabled.unwrap_or_default(),

            onclick: move |_| {
                pressed.toggle();
                if let Some(onchange) = onchange {
                    onchange.call(*pressed.read());
                }
            },

            ..button_and_global_attrs,

            {children}
        }
    }
}