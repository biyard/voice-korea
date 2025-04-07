use crate::routes::Route;

use super::*;
use bdk::prelude::*;

#[component]
pub fn DeliberationDetailSettingLayout(lang: Language) -> Element {
    rsx! {
        div {
            id: "deliberation-detail-setting-layout",
            class: "flex flex-col w-full gap-20",

            div { class: "w-full flex flex-row justify-between, gap-10",
                for n in DeliberationDetailSettingStep::variants(&lang) {
                    button { class: "border", {n} }
                }
            }

            Outlet::<Route> {}
        } // end of this page
    }
}
