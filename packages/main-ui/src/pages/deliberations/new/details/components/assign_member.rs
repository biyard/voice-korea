use bdk::prelude::*;

use crate::{
    components::expandable_card::ExpandableCard,
    pages::deliberations::new::components::email_dropdown::EmailDropdown,
};

#[component]
pub fn Member(
    lang: Language,

    total_committees: Vec<String>,
    selected_committees: Vec<String>,

    add_committee: EventHandler<String>,
    remove_committee: EventHandler<String>,
    clear_committee: EventHandler<MouseEvent>,
) -> Element {
    let tr: MemberTranslate = translate(&lang);

    rsx! {
        ExpandableCard { required: false, header: tr.title, description: tr.description,
            EmailDropdown {
                lang,
                id: "final-committee",
                hint: tr.search_committee,

                selected_committees,
                committees: total_committees,

                add_committee: move |email: String| {
                    add_committee.call(email);
                },
                remove_committee: move |email: String| {
                    remove_committee.call(email);
                },
                clear_committee: move |e| {
                    clear_committee.call(e);
                },
            }
        }
    }
}

translate! {
    MemberTranslate;

    title: {
        ko: "담당자 지정",
        en: "Designate a person in charge"
    }

    description: {
        ko: "각 단계별 역할을 수행할 담당자를 선택하여 공론 과정에서의 책임과 역할을 명확하게 할 수 있도록 설정합니다.",
        en: "Select a person to perform each step of the process to ensure that responsibilities and roles are clear during the public hearing."
    }

    search_committee: {
        ko: "공론 위원회에서 검색",
        en: "Search in the Deliberation Committee"
    }
}
