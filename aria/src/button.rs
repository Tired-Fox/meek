use dioxus::prelude::*;

#[component]
pub fn Button(
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    disabled: Option<bool>,

    onclick: Option<EventHandler<MouseEvent>>,
    ondoubleclick: Option<EventHandler<MouseEvent>>,
    onmousemove: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    onwheel: Option<EventHandler<WheelEvent>>,
    onkeydown:  Option<EventHandler<KeyboardEvent>>,
    onkeyup:    Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onfocusin:  Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,

    children: Element,
) -> Element {
    rsx! {
        button {
            disabled: disabled,
            aria_disabled: disabled,

            onclick: move |evt| if let Some(handler) = onclick.as_ref() { handler.call(evt); },
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