use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::v2::PublicOpinionInstitutionSummary;
use num_format::{Locale, ToFormattedString};

use crate::{
    components::icons::{auth::Auth, project::ProjectIcon, vote::Vote},
    pages::main::i18n::InstitutionBoxTranslate,
};

#[component]
pub fn InstitutionBox(lang: Language, institution: PublicOpinionInstitutionSummary) -> Element {
    let institution_badge_url = asset!("/public/images/institution_badge.png").to_string();
    let tr: InstitutionBoxTranslate = translate(&lang);

    rsx! {
        div { class: "relative flex flex-col w-full justify-start items-start rounded-[20px] bg-white shadow-[0px_8px_20px_rgba(148,176,214,0.25)]",
            div { class: "bg-gray-200 w-full h-[75px] rounded-t-[20px]" }

            div { class: "absolute flex flex-row w-[60px] h-[60px] bg-white justify-center items-center top-[45px] left-[15px] rounded-[10px]",
                img { src: institution_badge_url, width: 48, height: 48 }
            }

            div { class: "flex flex-col w-full justify-start items-start pt-[40px] pb-[12px] px-[16px] gap-[48px]",
                div { class: "flex flex-col w-full gap-[10px]",
                    div { class: "flex flex-row w-full gap-[6px]",
                        div { class: "font-bold text-[16px] text-[#222222] ", "{institution.name}" }
                        Auth { width: "24", height: "24" }
                    }
                    div { class: "font-normal text-[14px] text-[#555462] leading-[22.4px] line-clamp-4",
                        "{institution.description}"
                    }
                }

                div { class: "flex flex-row w-full justify-between items-center",
                    div { class: "flex flex-row flex-1 justify-start items-center gap-[5px]",
                        ProjectIcon { width: "18", height: "18" }

                        div { class: "flex flex-row gap-[4px]",
                            div { class: "font-normal text-[14px] text-[#222222] leading-[17px]",
                                "{tr.project}"
                            }
                            div { class: "font-bold text-[14px] text-[#222222] leading-[17px]",
                                {institution.num_of_projects.to_formatted_string(&Locale::en)}
                            }
                        }
                    }

                    div { class: "flex flex-row justify-start items-center gap-[5px]",
                        Vote { width: "18", height: "18" }

                        div { class: "flex flex-row gap-[4px]",
                            div { class: "font-normal text-[14px] text-[#222222] leading-[17px]",
                                "{tr.vote}"
                            }
                            div { class: "font-bold text-[14px] text-[#222222] leading-[17px]",
                                {institution.num_of_vote.to_formatted_string(&Locale::en)}
                            }
                        }
                    }
                }
            }
        }
    }
}
