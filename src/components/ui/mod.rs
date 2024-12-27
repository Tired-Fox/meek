mod toggle;
mod accordian;

use dioxus::prelude::IntoAttributeValue;
pub use toggle::*;
pub use accordian::*;

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