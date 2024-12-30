use dioxus::prelude::*;
use lucide_dioxus::Bold;
use meek::Button;
use meek::Toggle;
use meek::{ Accordian, AccordianItem, AccordianHeader, AccordianContent, AccordianTrigger };
use meek::{ AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogTitle, AlertDialogDescription, AlertDialogCancel, AlertDialogAction };

#[component]
pub fn Showcase() -> Element {
    let pressed = use_signal(|| false);

    let accordian_items = [
        ("item-1", "Item 1"),
        ("item-2", "Item 2"),
    ];

    let mut alert_open = use_signal(|| false);

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
                    pressed: pressed,
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
            Button {
                class: "border px-2 py-1",
                onclick: move |_| {
                    alert_open.toggle()
                },
                "Toggle Alert Open",
            }
            div {
                class: "p-4",
                h2 { class: "text-xl font-bold mb-2", "Alert Dialog" }
                AlertDialog {
                    open: alert_open,
                    AlertDialogTrigger {
                        class: "border px-2 py-1",
                        "Open Alert Dialog: {alert_open}"
                    }
                    AlertDialogContent { class: "w-50 h-50 rounded-md p-4",
                        AlertDialogTitle { class: "text-2xl font-bold",
                            "Alert Dialog"
                        }
                        AlertDialogDescription {
                            "This is an alert dialog. A modal that interrupts the user and expects a response."
                        }
                        div {
                            class: "mt-2 flex gap-2 justify-end",
                            AlertDialogCancel {
                                class: "border px-2 py-1 rounded-md hover:bg-gray-100 focus:bg-gray-300 focus:outline-none",
                                "Cancel"
                            }
                            AlertDialogAction {
                                class: "px-2 py-1 rounded-md bg-rose-600 text-white focus:outline-none hover:opacity-85 focus:opacity-85",
                                onclick: move |_| {
                                    println!("Alert Dialog Action, CLICKED!");
                                },
                                "Action"
                            }
                        }
                    }
                }
            }
        }
    }
}
