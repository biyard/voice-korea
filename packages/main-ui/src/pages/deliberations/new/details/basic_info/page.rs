use super::super::components::{IntroductionCard, Member};
use super::*;
use crate::pages::deliberations::new::{
    components::footer_buttons::FooterButtons, details::basic_info::components::material::Material,
};
use bdk::prelude::*;
use controller::*;
use i18n::*;
use models::{File, ResourceFileSummary};

#[component]
pub fn DeliberationBasicInfoSettingPage(lang: Language) -> Element {
    let mut ctrl = Controller::new(lang)?;
    let tr: BasicInfoTranslate = translate(&lang);
    let basic_info = ctrl.get_basic_info();

    let metadatas = ctrl.metadatas()?;

    let surveys = ctrl.surveys()?;

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-medium text-base text-text-black mb-10", {tr.post_setting} }
                div { class: "flex flex-col w-full justify-start items-start gap-20",
                    IntroductionCard {
                        lang,
                        rich_text_id: "basic_rich_text",
                        start_date_id: "basic_start_date",
                        end_date_id: "basic_end_date",
                        description: tr.introduction_description.to_string(),
                        text_value: basic_info.clone().title,
                        started_at: basic_info.clone().started_at,
                        ended_at: basic_info.clone().ended_at,
                        content: basic_info.clone().description,
                        set_title: move |title: String| {
                            ctrl.set_title(title);
                        },
                        set_description: move |description: String| {
                            ctrl.set_description(description);
                        },
                        set_start_date: move |timestamp: i64| {
                            ctrl.set_start_date(timestamp);
                        },
                        set_end_date: move |timestamp: i64| {
                            ctrl.set_end_date(timestamp);
                        },
                    }
                    Member {
                        lang,
                        total_committees: ctrl.committee_members(),
                        selected_committees: ctrl.get_selected_committee(),
                        add_committee: move |email: String| {
                            ctrl.add_committee(email);
                        },
                        remove_committee: move |email: String| {
                            ctrl.remove_committee(email);
                        },
                        clear_committee: move |_| {
                            ctrl.clear_committee();
                        },
                    }
                    Material {
                        lang,
                        metadatas,
                        resources: ctrl.get_selected_resources(),
                        total_surveys: surveys,
                        selected_surveys: ctrl.get_selected_surveys(),
                        create_resource: move |file: File| async move {
                            let _ = ctrl.create_resource(file).await;
                        },
                        remove_resource: move |id: i64| {
                            let _ = ctrl.delete_resource(id);
                        },
                        add_resource: move |resource: ResourceFileSummary| {
                            let _ = ctrl.add_resource(resource.into());
                        },
                        add_survey: move |id: i64| {
                            ctrl.add_survey(id);
                        },
                        remove_survey: move |id: i64| {
                            ctrl.remove_survey(id);
                        },
                        clear_survey: move |_| {
                            ctrl.clear_survey();
                        },
                    }
                }

                FooterButtons {
                    lang,
                    on_backward: move |_| {
                        ctrl.back();
                    },
                    on_temp_save: move |_| async move { ctrl.temp_save().await },
                    on_next: move |_| {
                        ctrl.next();
                    },
                    on_save: None,
                    next_valid: ctrl.is_valid(),
                }
            }
        }
    }
}
