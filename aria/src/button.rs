use dioxus::prelude::*;

/// An accessibility button
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// 
/// # Accessibility
/// 
/// **Keyboard Interaction**
/// 
/// - `<Space>`: Open/Close the dialog.
/// - `<Enter>`: Open/Close the dialog.
#[component]
pub fn Button(
    toggled: Option<bool>,
    onchange: Option<EventHandler<bool>>,

    disabled: Option<bool>,

    onclick: Option<EventHandler<MouseEvent>>,
    ondoubleclick: Option<EventHandler<MouseEvent>>,
    onmousemove: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,

    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut state = use_signal(|| toggled.unwrap_or_default());
    use_effect(use_reactive!(|toggled| {
        if let Some(toggled) = toggled {
            state.set(toggled);
        }
    }));

    let on_off = use_memo(move || {
        if *state.read() { "on" } else { "off" }
    });

    rsx! {
        button {
            disabled: disabled,
            aria_disabled: disabled,
            "data-disabled": disabled.unwrap_or_default(),

            aria_pressed: toggled.map(|_| state()).or_else(|| onchange.map(|_| state())),
            "data-state": toggled.map(|_| on_off()).or_else(|| onchange.map(|_| on_off())),

            onclick: move |evt| {
                if let Some(onchange) = onchange.as_ref() {
                    onchange.call(*state.read())
                }
                if let Some(handler) = onclick.as_ref() { handler.call(evt); }
            },
            ondoubleclick: move |evt| if let Some(handler) = ondoubleclick.as_ref() { handler.call(evt); },
            onmousemove: move |evt| if let Some(handler) = onmousemove.as_ref() { handler.call(evt); },
            onmousedown: move |evt| if let Some(handler) = onmousedown.as_ref() { handler.call(evt); },
            onmouseup: move |evt| if let Some(handler) = onmouseup.as_ref() { handler.call(evt); },
            onwheel: move |evt| if let Some(handler) = onwheel.as_ref() { handler.call(evt); },
            onkeydown: move |evt| if let Some(handler) = onkeydown.as_ref() { handler.call(evt); },
            onkeyup: move |evt| if let Some(handler) = onkeyup.as_ref() { handler.call(evt); },
            onkeypress: move |evt| if let Some(handler) = onkeypress.as_ref() { handler.call(evt); },
            onfocusin: move |evt| if let Some(handler) = onfocusin.as_ref() { handler.call(evt); },
            onfocusout: move |evt| if let Some(handler) = onfocusout.as_ref() { handler.call(evt); },

            ..attrs,

            {children}
        }
    }
}