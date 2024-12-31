use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::create_id;

#[derive(Default, Clone, PartialEq)]
pub(crate) struct Portals { 
    mappings: BTreeMap<String, Vec<String>>,
    entries: BTreeMap<String, Element>
}

impl Portals {
    fn insert(&mut self, name: Option<String>, id: &String, children: Element) {
        let name = name.unwrap_or_default();
        if !self.entries.contains_key(id) {
            if self.mappings.contains_key(&name) {
                self.mappings.get_mut(&name).unwrap().push(id.clone());
            } else {
                self.mappings.insert(name, Vec::from([id.clone()]));
            }
        }

        self.entries.insert(id.clone(), children);
    }

    fn remove(&mut self, name: Option<String>, id: String) {
        let name = name.unwrap_or_default();
        if self.entries.contains_key(&id) {
            self.entries.remove(&id);
        }

        if self.mappings.contains_key(&name) {
            if let Some(idx) = self.mappings.get(&name).unwrap().iter().position(|v| v == &id) {
                self.mappings.get_mut(&name).unwrap().remove(idx);
            }
        }
    }

    fn entries(&self, name: Option<String>) -> Vec<&Element> {
        let name = name.unwrap_or_default();
        let ids = self.mappings.get(&name).map(|v| v.into_iter().collect::<Vec<&String>>()).unwrap_or_default();
        ids.into_iter()
            .filter_map(|id| self.entries.get(id))
            .collect::<Vec<_>>()
    }
}

/// Mappings of key and instance id to [`dioxus::prelude::Element`]
pub(crate) static PORTALS: GlobalSignal<Portals> = Signal::global(Portals::default);

/// Teleport the child elements to where the `Portal` element is located
/// 
/// This component requires that a single `Portal` element is rendered otherwise none of the child
/// elements will be rendered.
/// 
/// WARNING: As of right now, with the limitations with Dioxus and how the context and scope works, any child elements
/// rendered through this component will lose context and gain the scope and context of the `Portal` where it is rendered.
#[component]
pub fn Teleport(
    #[props(into)]
    name: Option<String>,
    children: Element
) -> Element {
    let id = use_signal(|| create_id());

    // WARN: Child components will no longer inherit the scope and context of the parent
    // TODO: Pass scope/context down so it can be used
    use_effect(use_reactive((&name, &children), move |(name, children)| {
        PORTALS.write().insert(name, &*id.read(), rsx! { {children} });
    }));

    use_drop(move || {
        PORTALS.write().remove(name, id.read().clone());
    });

    VNode::empty()
}

/// Displays all teleported elements
/// 
/// Acts as a marker in the DOM where all teleported elements will be rendered.
/// Break the elements out of their parent component/element and move them to where this marker is placed.
/// 
/// WARNING: Multiple instances of this component will result in multiple renders of all teleported components.
#[component]
pub fn Portal(
    name: Option<String>
) -> Element {
    rsx! {
        for item in PORTALS.read().entries(name) {
            {item}
        }
    }
}