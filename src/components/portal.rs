use dioxus::prelude::*;

#[component]
pub fn Portal(
    #[props(into)]
    name: String,

    children: Element
) -> Element {
    use_memo(move || {
        if let Ok(children) = &children {
            println!("{children:?}");
        }
    });

    // Split the children and convert them and render them into web_sys
    use_effect(move || {
        let portal_id = format!("meek-portal:{name}");

        let eval = format!(r#"
            let portal = document.getElementById("{portal_id}");
            if (!portal) {{
                portal = document.createElement("div");
                portal.id = "{portal_id}";
                document.body.appendChild(portal);
                console.log(portal);
            }}
        "#);

        spawn(async move {
            document::eval(eval.as_str()).await.unwrap();
        });
    });

    VNode::empty()
}