use dioxus::prelude::*;

// TODO: Checkbox groups

#[derive(Clone, Copy, PartialEq, strum::EnumIs)]
pub enum CheckboxState {
    On,
    Off,
    Intermediate,
}
impl CheckboxState {
    fn state(&self) -> String {
        match self {
            Self::Intermediate => "intermediate".to_string(),
            Self::On => "checked".to_string(),
            Self::Off => "unchecked".to_string(),
        }
    }

    fn aria(&self) -> String {
        match self {
            Self::Intermediate => "mixed".to_string(),
            Self::On => "true".to_string(),
            Self::Off => "false".to_string(),
        }
    }

    fn value(&self) -> String {
        match self {
            Self::Intermediate => "intermediate".to_string(),
            Self::On => "on".to_string(),
            Self::Off => "off".to_string(),
        }
    }
}
impl<A: AsRef<str>> From<A> for CheckboxState {
    fn from(value: A) -> Self {
        match value.as_ref().to_ascii_lowercase().as_str() {
            "intermediate" =>  Self::Intermediate,
            "on" | "true" => Self::On,
            "off" | "false" => Self::Off,
            other => panic!("unknown checkbox state value: {other}")
        }
    }
}

#[component]
pub fn Checkbox(
    #[props(into)]
    checked: Option<CheckboxState>,
    #[props(into)]
    default: Option<CheckboxState>,

    onchange: Option<EventHandler<CheckboxState>>,

    #[props(into)]
    aria_labelledby: Option<String>,
    #[props(into)]
    aria_label: Option<String>,

    #[props(into)]
    name: Option<String>,
    disabled: Option<bool>,

    #[props(extends=Input, extends=GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut state = use_signal(|| default.unwrap_or(CheckboxState::Off));
    use_effect(use_reactive!(|checked| {
        if let Some(checked) = checked {
            state.set(checked);
        }
    }));

    rsx!{
        button {
            role: "checkbox",
            aria_label,
            aria_labelledby,

            "data-state": state.read().state(),
            aria_checked: state.read().aria(),
            value: state.read().value(),

            disabled,
            aria_disabled: disabled,

            onclick: move |_| {
                match state() {
                    CheckboxState::Intermediate => state.set(CheckboxState::On),
                    CheckboxState::On => state.set(CheckboxState::Off),
                    CheckboxState::Off => state.set(CheckboxState::On),
                }

                if let Some(onchange) = onchange.as_ref() {
                    onchange.call(state());
                }
            },

            ..attrs,

            {children}
        }
        input {
            r#type: "checkbox",
            aria_hidden: true,
            tabindex: -1,
            value: state.read().value(),
            name,
            disabled,

            transform: "translateX(-100%)",
            position: "absolute",
            pointer_events: "none",
            opacity: 0,
            margin: "0px",
            width: "0px",
            height: "0px",
        }
    }
}