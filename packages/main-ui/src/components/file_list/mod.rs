use crate::components::icons::{CloseWithBackGround, Docs, Jpg, Pdf, Png, Pptx, Xlsx, Zip};
use dioxus::prelude::*;
use models::{File, FileExtension};

#[component]
pub fn FileList(items: Vec<File>, onremove: EventHandler<usize>) -> Element {
    rsx! {
        div { class: "w-full h-full overflow-y-auto flex flex-col gap-2.5 max-h-[170px] pr-2.5",
            for (index , item) in items.iter().enumerate() {
                div { class: "w-full px-4 py-3 flex flex-row text-xs gap-2 rounded-lg items-center border border-[#E7E7E7] ",
                    if item.ext == FileExtension::JPG {
                        Jpg {}
                    } else if item.ext == FileExtension::PNG {
                        Png {}
                    } else if item.ext == FileExtension::PDF {
                        Pdf {}
                    } else if item.ext == FileExtension::ZIP {
                        Zip {}
                    } else if item.ext == FileExtension::WORD {
                        Docs {}
                    } else if item.ext == FileExtension::PPTX {
                        Pptx {}
                    } else {
                        Xlsx {}
                    }

                    div { class: "text-xs flex-1",
                        p { class: "text-[#0b0b0b] font-semibold leading-[18px]",
                            "{item.name}"
                        }
                        p { class: "text-[#6d6d6d]", "{item.size}" }
                    }
                    button {
                        onclick: move |_| {
                            onremove.call(index);
                        },
                        CloseWithBackGround {}
                    }
                }
            }
        }
    }
}
