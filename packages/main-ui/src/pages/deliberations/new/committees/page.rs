#![allow(non_snake_case)]
use super::*;
use bdk::prelude::*;
use controller::*;
use i18n::*;

use models::deliberation_user::DeliberationUserCreateRequest;

use crate::pages::deliberations::new::components::role_dropdown::RoleDropdown;

#[component]
pub fn CompositionCommitee(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: CompositionCommitteeTranslate = translate(&lang);

    let roles = ctrl.roles();
    let members = ctrl.members()?;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-base text-text-black mb-10",
                {tr.composition_committee_title}
            }

            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-40 py-24",
                div { class: "font-bold text-text-black text-lg mb-3", {tr.division_roles} }
                div { class: "font-normal text-text-gray text-sm mb-20",
                    {tr.composition_committee_description}
                }
                // selection box
                div { class: "flex flex-col w-full justify-start items-center mb-40",
                    for (i , committee_role) in ctrl.committee_roles().iter().enumerate() {
                        RoleDropdown {
                            id: format!("{}_dropdown", roles[i].to_string()),
                            label: roles[i].translate(&lang),
                            hint: tr.enter_charge_person,
                            total_committees: ctrl.committees(),
                            members: members.clone(),
                            committees: committee_role.clone(),
                            add_committee: {
                                let role = roles[i].clone();
                                move |user_id: i64| {
                                    ctrl.add_committee(DeliberationUserCreateRequest {
                                        user_id,
                                        role: role.clone(),
                                    });
                                    ctrl.add_committee_roles(i, user_id);
                                }
                            },
                            remove_committee: {
                                let role = roles[i].clone();
                                move |user_id: i64| {
                                    ctrl.remove_committee(user_id, role.clone());
                                    ctrl.remove_committee_roles(i, user_id);
                                }
                            },
                            clear_committee: {
                                let role = roles[i].clone();
                                move |_| {
                                    ctrl.clear_committee(role.clone());
                                    ctrl.clear_committee_roles(i);
                                }
                            },
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-40 mb-50",
                div {
                    class: "flex flex-row w-70 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {
                        ctrl.save_deliberation();
                        ctrl.back();
                    },
                    {tr.backward}
                }
                div {
                    class: "flex flex-row w-105 h-55 rounded-sm justify-center items-center bg-white border border-label-border-gray font-semibold text-base text-table-text-gray mr-20",
                    onclick: move |_| {},
                    {tr.temporary_save}
                }
                div {
                    class: "cursor-pointer flex flex-row w-110 h-55 rounded-sm justify-center items-center bg-hover font-semibold text-base text-white",
                    onclick: move |_| {
                        ctrl.save_deliberation();
                        ctrl.next();
                    },
                    {tr.next}
                }
            }
        }
    }
}
