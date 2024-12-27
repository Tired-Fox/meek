use std::{collections::HashSet, sync::atomic::AtomicUsize};

use dioxus::prelude::*;

use super::Orientation;

const ACCORDIAN_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Default, Debug, Clone, Copy, PartialEq, strum::EnumIs)]
pub enum AccordianType {
    #[default]
    Single,
    Multiple
}

impl<A: AsRef<str>> From<A> for AccordianType {
    fn from(value: A) -> Self {
        match value.as_ref().to_ascii_lowercase().as_str() {
            "single" => Self::Single,
            "multiple" => Self::Multiple,
            other => panic!("unknown accordian type: '{other}'")
        }
    }
}

/// Handles the contextual state of an accordian
#[derive(Debug, Clone, PartialEq)]
pub struct AccordianState {
    pub collapsible: bool,
    pub typ: AccordianType,
    pub orientation: Orientation,

    current: HashSet<String>,
    onchange: Option<EventHandler<HashSet<String>>>,
}

impl AccordianState {
    fn new<S: std::fmt::Display, I: IntoIterator<Item=S>>(
        collapsible: Option<bool>,
        typ: AccordianType,
        orientation: Option<Orientation>,
        default: Option<I>,
        onchange: Option<EventHandler<HashSet<String>>>,
    ) -> Self {

        let current: HashSet<String> = if typ.is_multiple() {
            default.map(|d| d.into_iter().map(|v| v.to_string()).collect()).unwrap_or_default()
        } else {
            default.map(|d| d.into_iter().take(1).map(|v| v.to_string()).collect()).unwrap_or_default()
        };

        Self {
            orientation: orientation.unwrap_or(Orientation::Vertical),
            collapsible: collapsible.unwrap_or_default(),
            typ,
            current,
            onchange,
        }
    }

    /// Toggle an item's open state
    pub fn toggle(&mut self, new: impl AsRef<str>) {
        let new = new.as_ref();
        if self.contains(&new) && ((self.typ.is_single() && self.collapsible) || self.typ.is_multiple()) {
            self.current.remove(new);
            if let Some(onchange) = self.onchange.as_ref() {
                onchange.call(self.current.clone());
            }
        } else {
            if !self.typ.is_multiple() && !self.current.is_empty() {
                self.current.clear();
            }

            self.current.insert(new.to_string());
            if let Some(onchange) = self.onchange.as_ref() {
                onchange.call(self.current.clone());
            }
        }
    }

    /// Check if an item's value is currently in the open state
    pub fn contains(&self, value: impl AsRef<str>) -> bool {
        self.current.contains(value.as_ref())
    }
}

/// Accordian
/// 
/// Stacked set of interactive headings that reveal an associated section of content when selected.
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// - `[data-orientation]`: `"vertical"` | `"horizontal"`
/// 
/// # Accessibility
/// 
/// **Keyboard Interaction**
/// 
/// - `<Tab>`: Change focus between the accordian item headers/triggers.
/// - `<Shift + Tab>`: Change focus between the accordian item headers/triggers.
#[component]
pub fn Accordian(
    /// Default item(s) to open
    #[props(into)]
    default: Option<Vec<String>>,
    /// Whether a "single" type accordian allows the user to close an open item
    collapsible: Option<bool>,
    /// Whether an accordian supports
    #[props(into, default = AccordianType::Single)]
    r#type: AccordianType,
    orientation: Option<Orientation>,

    onchange: Option<EventHandler<HashSet<String>>>,

    children: Element
) -> Element {
    use_context_provider(|| Signal::new(AccordianState::new(collapsible, r#type, orientation, default, onchange)));

    rsx! {
        div {
            "data-orientation": orientation.unwrap_or(Orientation::Vertical),

            {children}
        }
    }
}

/// Handles the contextual state of a accordian item
#[derive(Debug, Clone, PartialEq)]
pub struct AccordianItemState {
    value: String,
    id: usize,
    pub disabled: bool,
}

impl AccordianItemState {
    fn new(value: impl std::fmt::Display, id: usize, disabled: bool) -> Self {
        Self {
            value: value.to_string(),
            id,
            disabled
        }
    }

    /// Get the formatted trigger id
    fn trigger_id(&self) -> String {
        format!("meek-accordian-:t{}:", self.id)
    }

    /// Get the formatted content id
    fn content_id(&self) -> String {
        format!("meek-accordian-:i{}:", self.id)
    }

    /// Get the items value
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Contains the context and parts for a single collapsible section
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// - `[data-orientation]`: `"vertical"` | `"horizontal"`
/// - `[data-disabled]`: Present when disabled
#[component]
pub fn AccordianItem(
    #[props(into)]
    value: String,
    disabled: Option<bool>,
    children: Element,

    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let state = use_context::<Signal<AccordianState>>();
    let id = ACCORDIAN_ID.load(std::sync::atomic::Ordering::Acquire).saturating_add(1);

    ACCORDIAN_ID.store(id, std::sync::atomic::Ordering::Release);
    use_context_provider(|| AccordianItemState::new(value.clone(), id, disabled.unwrap_or_default()));

    rsx! {
        div {
            "data-state": if state.read().contains(&value) { "open" } else { "closed" },
            "data-disabled": state.read().contains(&value) && !state.read().collapsible,
            "data-orientation": state.read().orientation,

            ..attrs,
            {children}
        }
    }
}

/// Wraps AccordianTrigger with a heading tag
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// - `[data-orientation]`: `"vertical"` | `"horizontal"`
/// - `[data-disabled]`: Present when disabled
#[component]
pub fn AccordianHeader(
    children: Element,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let state = use_context::<Signal<AccordianState>>();

    rsx! {
        h3 {
            "data-orientation": state.read().orientation,

            ..attrs,
            {children}
        }
    }
}

/// Toggles the collapsed state of its associated item. It should be nested inside of AccordianHeader for accessibility
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// - `[data-orientation]`: `"vertical"` | `"horizontal"`
/// - `[data-disabled]`: Present when disabled
/// 
/// # Accessibility
/// 
/// **Keyboard Interaction**
/// 
/// - `<Space>`: Opens/Closes the item.
/// - `<Enter>`: Opens/Closes the item.
#[component]
pub fn AccordianTrigger(
    children: Element,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let mut state = use_context::<Signal<AccordianState>>();
    let id = use_context::<AccordianItemState>();

    rsx! {
        button {
            r#type: "button",
            id: id.trigger_id(),
            aria_controls: id.content_id(),
            aria_expanded: state.read().contains(id.value()),
            aria_disabled: state.read().contains(id.value()) && !state.read().collapsible,
            disabled: state.read().contains(id.value()) && !state.read().collapsible,

            "data-state": if state.read().contains(id.value()) { "open" } else { "closed" },
            "data-orientation": state.read().orientation,

            onclick: move |_| state.write().toggle(id.value()),

            ..attrs,

            {children}
        }
    }
}

/// Contains the collapsible content for an item
/// 
/// # Data Attributes
/// 
/// - `[data-state]`: `"open"` | `"closed"`
/// - `[data-orientation]`: `"vertical"` | `"horizontal"`
/// - `[data-disabled]`: Present when disabled
#[component]
pub fn AccordianContent(
    children: Element
) -> Element {
    let state = use_context::<Signal<AccordianState>>();
    let id = use_context::<AccordianItemState>();

    rsx! {
        div {
            id: id.content_id(),
            role: if state.read().typ.is_multiple() { None } else { Some("region") },
            aria_labelledby: id.trigger_id(),

            hidden: !state.read().contains(id.value()),

            "data-state": if state.read().contains(id.value()) { "open" } else { "closed" },
            "data-orientation": state.read().orientation,

            {children}
        }
    }
}