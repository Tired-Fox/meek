mod toggle;
mod accordian;
mod alert_dialog;
mod portal;
mod button;

pub use toggle::*;
pub use accordian::*;
pub use alert_dialog::*;
pub use portal::*;
pub use button::*;

use dioxus::prelude::*;

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