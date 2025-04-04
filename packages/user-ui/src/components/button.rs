use bdk::prelude::*;

#[component]
pub fn Button(
    onclick: EventHandler<MouseEvent>,
    #[props(default = "".to_string())] class: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        button {
            onclick: move |e| {
                onclick.call(e);
            },
            class: format!(
                "flex items-center justify-center bg-bt-primary cursor-pointer hover:bg-gradient-to-t hover:from-black/20 hover:to-black/20 disabled:bg-bt-disabled disabled:cursor-not-allowed {class}",
            ),
            ..attributes,
            {children}
        }
    }
}
