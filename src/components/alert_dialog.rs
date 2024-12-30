use dioxus::prelude::*;

use super::MaybeSignal;
use super::Button;

struct AlertDialogContext {
    pub(crate) id: String,
    pub(crate) title_id: Option<String>,
    pub(crate) description_id: Option<String>,

    pub(crate) open: Signal<bool>,
    pub(crate) onchange: Option<EventHandler<bool>>,
}

impl AlertDialogContext {
    fn new(id: Option<String>, open: Signal<bool>, onchange: Option<EventHandler<bool>>) -> Self {
        Self {
            id: id.unwrap_or(short_uuid::ShortUuid::from_uuid(&uuid::Uuid::new_v4()).to_string()),
            title_id: None,
            description_id: None,

            open,
            onchange,
        }
    }

    fn set_title(&mut self, id: Option<String>) {
        self.title_id.replace(id.unwrap_or(format!("{}:title", self.id)));
    }

    fn set_description(&mut self, id: Option<String>) {
        self.description_id.replace(id.unwrap_or(format!("{}:description", self.id)));
    }

    fn close(&mut self) {
        *self.open.write() = false;
        if let Some(onchange) = self.onchange.as_ref() {
            onchange.call(false);
        }
    }

    fn open(&mut self) {
        *self.open.write() = true;
        if let Some(onchange) = self.onchange.as_ref() {
            onchange.call(true);
        }
    }

    fn show(&self) {
        let id = self.id.clone();
        spawn(async move {
            let eval = format!(r#"document.getElementById("{id}")?.showModal()"#);
            document::eval(eval.as_str()).await.unwrap();
        });
    }

    fn hide(&self) {
        let id = self.id.clone();
        spawn(async move {
            let eval = format!(r#"document.getElementById("{id}")?.close()"#);
            document::eval(eval.as_str()).await.unwrap();
        });
    }
}

#[derive(Clone, PartialEq, Props)]
struct DProps {
    /// Passed state whether the modal should be open or closed
    #[props(into)]
    open: Option<MaybeSignal<bool>>,
}

/// A modal dialog that interrupts the user with important content that expects a response
#[component]
pub fn AlertDialog(
    /// Passed state whether the modal should be open or closed
    #[props(into, default = MaybeSignal::new(false))]
    open: MaybeSignal<bool>,
    /// Default open state of the modal
    default: Option<bool>,
    /// Event handler for when the open state changes
    onchange: Option<EventHandler<bool>>,
    /// User defined ID of the dialog
    #[props(into)]
    id: Option<String>,

    children: Element,
) -> Element {
    let open = open.as_signal();
    use_context_provider(|| Signal::new(AlertDialogContext::new(id, open, onchange)));
    // Update showing and hiding the modal based on the `open` state
    use_effect(move || {
        let mut context =  use_context::<Signal<AlertDialogContext>>();
        if *open.read() { context.write().show() }
        else { context.write().hide() }
    });

    rsx! {
        {children}
    }
}

/// A button that opens the dialog
#[component]
pub fn AlertDialogTrigger(
    disabled: Option<bool>,

    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();
    rsx! {
        button {
            r#type: "button",
            disabled: disabled,
            aria_disabled: disabled,

            "data-state": if *context.read().open.read() { "open" } else { "closed" },

            onclick: move |_| context.write().open(),

            ..attrs,

            {children}
        }
    }
}

/// A `dialog` html element which contains the content to be rendered when it is open
#[component]
pub fn AlertDialogContent(
    #[props(into)]
    aria_labelledby: Option<String>,
    #[props(into)]
    aria_describedby: Option<String>,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();

    let onkeydown = move |evt: Event<KeyboardData>| {
        match evt.key() {
            Key::Escape => context.write().close(),
            _ => {}
        }
    };

    rsx! {
        dialog {
            id: context.read().id.as_str(),
            role: "alertdialog",
            aria_labelledby: aria_labelledby.as_deref().or(context.read().title_id.as_deref()),
            aria_describedby: aria_describedby.as_deref().or(context.read().description_id.as_deref()),
            "data-state": if *context.read().open.read() { "open" } else { "closed" },

            onkeydown: onkeydown,

            ..attrs,

            {children}
        }
    }
}

/// An accessible name to be announced when the dialog opens
/// 
/// Alternatively, `aria-label` or `aria-labelledby` can be provided to `AlertDialogContext` and this component can be excluded.
#[component]
pub fn AlertDialogTitle(
    #[props(into)]
    id: Option<String>,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();

    use_effect(move || {
        context.write().set_title(id.clone());
    });

    rsx! {
        h2 {
            ..attrs,
            {children}
        }
    }
}

/// An accessible description to be announced when the dialog is opened
/// 
/// Alternatively, `aria-describedby` can be provided to `AlertDialogContent` and this component can be exluded.
#[component]
pub fn AlertDialogDescription(
    #[props(into)]
    id: Option<String>,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();

    use_effect(move || {
        context.write().set_description(id.clone());
    });

    rsx! {
        div {
            ..attrs,
            {children}
        }
    }
}

/// A button that closes the dialog
/// 
/// This button should be distinguished from `AlertDialogAction` visually
#[component]
pub fn AlertDialogCancel(
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();

    rsx! {
        Button {
            onclick: move |_| context.write().close(),
            ..attrs,
            {children}
        }
    }
}

/// A button that closes the dialog.
/// 
/// This button should be distinguished from `AlertDialogCancel` visually
#[component]
pub fn AlertDialogAction(
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>,

    onclick: Option<EventHandler<Event<MouseData>>>,

    children: Element,
) -> Element {
    let mut context =  use_context::<Signal<AlertDialogContext>>();

    rsx! {
        button {
            onclick: move |evt| {
                context.write().close();
                if let Some(onclick) = onclick.as_ref() {
                    onclick.call(evt);
                }
            },
            ..attrs,
            {children}
        }
    }
}