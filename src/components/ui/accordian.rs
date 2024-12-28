use std::{collections::{BTreeMap, HashSet}, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use dioxus::prelude::*;

use super::Orientation;

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
#[derive(Debug)]
pub struct AccordianState {
    pub collapsible: bool,
    pub typ: AccordianType,
    pub orientation: Orientation,

    items: Vec<Rc<MountedData>>,
    item_map: BTreeMap<String, usize>,

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
            items: Default::default(),
            item_map: Default::default(),
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

    fn add_item(&mut self, key: impl AsRef<str>, data: Rc<MountedData>) {
        let key = key.as_ref();
        if !self.item_map.contains_key(key) {
            self.item_map.insert(key.to_string(), self.items.len());
            self.items.push(data);
        }
    }

    async fn handle_key(&self, key: impl AsRef<str>, evt: Event<KeyboardData>) {
        let key = key.as_ref();

        match evt.key() {
            Key::ArrowUp => if self.orientation.is_vertical() {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index > 0 {
                        self.items[index-1].set_focus(true).await;
                    }
                }
            },
            Key::ArrowDown => if self.orientation.is_vertical() {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index < self.items.len() - 1 {
                        self.items[index+1].set_focus(true).await;
                    }
                }
            },
            Key::ArrowRight => if self.orientation.is_horizontal() {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index < self.items.len() - 1 {
                        self.items[index+1].set_focus(true).await;
                    }
                }
            },
            Key::ArrowLeft => if self.orientation.is_horizontal() {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index > 0 {
                        self.items[index-1].set_focus(true).await;
                    }
                }
            },
            Key::Home => {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index != 0 {
                        self.items[0].set_focus(true).await;
                    }
                }
            },
            Key::End => {
                evt.prevent_default();
                if let Some(index) = self.item_map.get(key) {
                    if *index != self.items.len() - 1 {
                        self.items[self.items.len() - 1].set_focus(true).await;
                    }
                }
            },
            _ => {}
        }
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
    id: String,
    trigger_id: Option<String>,
    content_id: Option<String>,
    pub disabled: bool,
}

impl AccordianItemState {
    fn new(value: impl std::fmt::Display, id: Option<String>, disabled: bool) -> Self {
        Self {
            value: value.to_string(),
            id: id.unwrap_or(short_uuid::ShortUuid::from_uuid(&uuid::Uuid::new_v4()).to_string()),
            trigger_id: None,
            content_id: None,
            disabled
        }
    }

    /// Get the formatted trigger id
    fn trigger_id(&self) -> String {
        self.trigger_id.clone()
            .unwrap_or(format!("meek-accordian-trigger-{}", self.id))
    }

    /// Get the formatted content id
    fn content_id(&self) -> String {
        self.content_id.clone()
            .unwrap_or(format!("meek-accordian-content-{}", self.id))
    }

    fn id(&self) -> String {
        format!("meek-accordian-:{}:", self.id)
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
    #[props(into)]
    id: Option<String>,
    children: Element,

    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let state = use_context::<Signal<AccordianState>>();

    use_context_provider(|| Signal::new(AccordianItemState::new(value.clone(), id, disabled.unwrap_or_default())));
    let id = use_context::<Signal<AccordianItemState>>();

    rsx! {
        div {
            id: id.read().id(),
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
/// - `<ArrowDown>`: [Vertical] Focus next item. If the last item is currently focused, this will do nothing.
/// - `<ArrowUp>`: [Vertical] Focus previous item. If the first item is currently focused, this will do nothing.
/// - `<ArrowRight>`: [Horizontal] Focus next item. If the last item is currently focused, this will do nothing.
/// - `<ArrowLeft>`: [Horizontal] Focus previous item. If the first item is currently focused, this will do nothing.
/// - `<Home>`: Focus the first item. If the last item is currently focused, this will do nothing.
/// - `<End>`: Focus the last item. If the first item is currently focused, this will do nothing.
#[component]
pub fn AccordianTrigger(
    #[props(into)]
    id: Option<String>,
    children: Element,
    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let mut state = use_context::<Signal<AccordianState>>();
    let mut iid = use_context::<Signal<AccordianItemState>>();

    use_effect(move || {
        if let Some(id) = &id {
            iid.write().trigger_id = Some(id.to_string());
        }
    });

    rsx! {
        button {
            r#type: "button",
            id: iid.read().trigger_id(),
            aria_controls: iid.read().content_id(),
            aria_expanded: state.read().contains(iid.read().value()),
            aria_disabled: state.read().contains(iid.read().value()) && !state.read().collapsible,
            disabled: state.read().contains(iid.read().value()) && !state.read().collapsible,

            "data-state": if state.read().contains(iid.read().value()) { "open" } else { "closed" },
            "data-orientation": state.read().orientation,

            onclick: move |_: Event<MouseData>| state.write().toggle(iid.read().value()),
            onkeydown: move |evt| async move { state.read().handle_key(iid.read().value(), evt).await },

            onmounted: move |v: Event<MountedData>| state.write().add_item(iid.read().value(), v.data()),

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
    #[props(into)]
    id: Option<String>,
    children: Element,

    #[props(extends = GlobalAttributes)]
    attrs: Vec<Attribute>
) -> Element {
    let state = use_context::<Signal<AccordianState>>();
    let mut iid = use_context::<Signal<AccordianItemState>>();

    if let Some(id) = id {
        iid.write().content_id = Some(id);
    }

    rsx! {
        div {
            id: iid.read().content_id(),
            role: if state.read().typ.is_multiple() { None } else { Some("region") },
            aria_labelledby: iid.read().trigger_id(),

            hidden: !state.read().contains(iid.read().value()),

            "data-state": if state.read().contains(iid.read().value()) { "open" } else { "closed" },
            "data-orientation": state.read().orientation,

            ..attrs,

            {children}
        }
    }
}
