use crate::pages::{
    components::{institution_box::InstitutionBox, search_project::SearchProject},
    governance::controller::Controller,
    GovernanceListTranslate, OrgSorter,
};
use bdk::prelude::*;
use models::organization::{OrganizationSorter, OrganizationSummary};
#[component]
pub fn GovernanceListPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: GovernanceListTranslate = translate(&lang);

    let organizations = ctrl.organizations()?.items;

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center",
            div { class: "max-w-desktop flex flex-col w-full justify-start items-start gap-20",
                div { class: "flex flex-row w-full justify-start items-start gap-15",
                    SearchProject {
                        lang,
                        placeholder: tr.search,
                        onsearch: move |title: String| {
                            ctrl.search_keyword.set(title);
                        },
                    }

                    div { class: " w-full flex flex-row justify-end items-center",
                        OrgSorter {
                            id: "organization_sorter_dropdown",
                            lang,
                            sorter: ctrl.sorter(),
                            on_sorter_changed: move |sorter: OrganizationSorter| {
                                ctrl.sorter.set(sorter);
                            },
                        }
                    }
                }

                GovernanceList { lang, organizations }
            }
        }
    }
}

#[component]
pub fn GovernanceList(lang: Language, organizations: Vec<OrganizationSummary>) -> Element {
    let tr: GovernanceListTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full justify-center items-center gap-10",
            div { class: "flex flex-row w-full justify-start items-start font-semibold text-lg text-black",
                {tr.governance}
            }

            div { class: "grid grid-cols-1 tablet:grid-cols-2 desktop:grid-cols-3 gap-20 w-full mt-30 [&>:nth-child(n+3)]:hidden tablet:[&>:nth-child(n+3)]:block tablet:[&>:nth-child(n+5)]:hidden desktop:[&>*]:!block",
                for organization in organizations {
                    InstitutionBox { lang, institution: organization }
                }
            }
        }
    }
}
