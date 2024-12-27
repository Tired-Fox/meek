use dioxus::prelude::*;

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

    onclick: Option<EventHandler<MouseEvent>>,

    children: Option<Element>,

    #[props(extends = GlobalAttributes, extends = Button)]
    button_and_global_attrs: Vec<Attribute>,
) -> Element {
    rsx! {
        button {
            r#type: "button",

            aria_pressed: pressed,
            "data-state": if let Some(true) = pressed { "on" } else { "off" },

            disabled: disabled.unwrap_or_default(),
            "data-disabled": disabled.unwrap_or_default(),

            onclick: move |evt| if let Some(onclick) = onclick { onclick.call(evt); },

            ..button_and_global_attrs,

            {children}
        }
    }
}