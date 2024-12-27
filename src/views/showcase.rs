use dioxus::prelude::*;
use lucide_dioxus::Bold;
use meek::ui::{AccordianType, Toggle};
use meek::ui::{ Accordian, AccordianItem, AccordianHeader, AccordianContent, AccordianTrigger };

#[component]
pub fn Showcase() -> Element {
    let mut pressed = use_signal(|| false);

    rsx! {
        main {
            style: r#"
                width: 100vw;
                height: 100vh;
                background-color: black;
                color: white;
            "#,
            style {r#"
                button[data-state="on"] {{
                    background-color: red;
                    border-color: red;
                }}
            "#}
            Toggle {
                class: "",
                pressed: pressed(),
                onclick: move |_| pressed.toggle(),
                aria_label: "Toggle bold",
                Bold { class: "w-4 h-4" }
            }
            Accordian {
                r#type: "multiple",
                collapsible: true,
                onchange: move |items| {
                    println!("{items:?}");
                },
                AccordianItem {
                    value: "item-1",
                    AccordianHeader { 
                        AccordianTrigger {
                            "Item 1"
                        }
                    }
                    AccordianContent {
                        "Item 1 Content"
                    }
                }
                AccordianItem {
                    value: "item-2",
                    AccordianHeader { 
                        AccordianTrigger {
                            "Item 2"
                        }
                    }
                    AccordianContent {
                        "Item 2 Content"
                    }
                }
            }
        }
    }
}
