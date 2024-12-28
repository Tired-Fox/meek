use dioxus::prelude::*;
use lucide_dioxus::Bold;
use meek::ui::Toggle;
use meek::ui::{ Accordian, AccordianItem, AccordianHeader, AccordianContent, AccordianTrigger };

#[component]
pub fn Showcase() -> Element {
    let mut pressed = use_signal(|| false);

    let accordian_items = [
        ("item-1", "Item 1"),
        ("item-2", "Item 2"),
    ];

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
            div {
                class: "p-4",
                h2 { class: "text-xl font-bold mb-2", "Accordian" }
                Toggle {
                    class: "",
                    pressed: pressed(),
                    onclick: move |_| pressed.toggle(),
                    aria_label: "Toggle bold",
                    Bold { class: "w-4 h-4" }
                }
            }
            div {
                class: "p-4",
                h2 { class: "text-xl font-bold mb-2", "Accordian" }
                Accordian {
                    collapsible: true,
                    onchange: move |items| {
                        println!("{items:?}");
                    },
                    for (i, item) in accordian_items.iter().enumerate() {
                        AccordianItem {
                            id: format!("accord-{i}"),
                            key: item.0,
                            value: item.0,
                            AccordianHeader { 
                                AccordianTrigger {
                                    id: format!("accord-{i}-trigger"),
                                    class: "group w-full border flex justify-between px-6 focus-visible:border-rose-500 outline-none focus:outline-none",
                                    {item.1}
                                    span { class: "group-data-[state=open]:hidden", ">" }
                                    span { class: "group-data-[state=closed]:hidden", "v" }
                                }
                            }
                            AccordianContent {
                                id: format!("accord-{i}-content"),
                                class: "border border-sky-700 p-2",
                                "{item.1} Content"
                            }
                        }
                    }
                }
            }
        }
    }
}
