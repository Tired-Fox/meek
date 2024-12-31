mod accordian;
mod alert_dialog;
mod portal;
mod button;
mod checkbox;
mod optional;

use std::sync::atomic::AtomicUsize;

pub use optional::Optional;
pub use accordian::*;
pub use alert_dialog::*;
pub use portal::*;
pub use button::*;
pub use checkbox::*;

use dioxus::prelude::*;

#[inline]
pub fn create_id() -> String {
    static ID_COUNT: AtomicUsize = AtomicUsize::new(1);

    format!(
        "mxa-{}",
        ID_COUNT
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            .to_string()
    )
}

/// Value that is possibly a reactive signal
#[derive(Clone, PartialEq, strum::EnumIs)]
pub enum MaybeSignal<T: 'static> {
    Reactive(Signal<T>),
    Normal(T),
}
impl<T> MaybeSignal<T> {
    pub fn signal(value: T) -> Self {
        Self::Reactive(Signal::new(value))
    }

    pub fn new(value: T) -> Self {
        Self::Normal(value)
    }

    pub fn as_signal(self) -> Signal<T> {
        match self {
            Self::Normal(value) => Signal::new(value),
            Self::Reactive(signal) => signal
        }
    }
}
impl<T> From<T> for MaybeSignal<T> {
    fn from(value: T) -> Self {
        MaybeSignal::Normal(value)
    }
}
impl<T> From<Signal<T>> for MaybeSignal<T> {
    fn from(value: Signal<T>) -> Self {
        MaybeSignal::Reactive(value)
    }
}

/// Value that is possibly a reactive signal
#[derive(Default, Clone, PartialEq, strum::EnumIs)]
pub enum OptionalMaybeSignal<T: 'static> {
    Reactive(Signal<T>),
    Normal(T),
    #[default]
    None,
}
impl<T> OptionalMaybeSignal<T> {
    pub fn signal(value: T) -> Self {
        Self::Reactive(Signal::new(value))
    }

    pub fn new(value: T) -> Self {
        Self::Normal(value)
    }

    pub fn as_signal(self, default: T) -> Signal<T> {
        match self {
            Self::Normal(value) => use_signal(|| value),
            Self::Reactive(signal) => signal,
            Self::None => use_signal(|| default),
        }
    }
}
impl<T> From<T> for OptionalMaybeSignal<T> {
    fn from(value: T) -> Self {
        OptionalMaybeSignal::Normal(value)
    }
}
impl<T> From<Signal<T>> for OptionalMaybeSignal<T> {
    fn from(value: Signal<T>) -> Self {
        OptionalMaybeSignal::Reactive(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIs)]
pub enum Orientation {
    Vertical,
    Horizontal
}
impl std::fmt::Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Vertical => write!(f, "vertical"),
            Self::Horizontal => write!(f, "horizontal"),
        }
    }
}
impl IntoAttributeValue for Orientation {
    fn into_value(self) -> dioxus::dioxus_core::AttributeValue {
        dioxus::dioxus_core::AttributeValue::Text(self.to_string())
    }
}

#[component]
pub fn Provider<T: Clone + PartialEq + 'static>(
    inherit: T,
    children: Element
) -> Element {
    use_context_provider(|| inherit);
    rsx! {
        {children}
    }
}