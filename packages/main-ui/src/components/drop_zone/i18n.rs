use dioxus_translate::translate;

translate! {
    DropZoneTranslate;

    description: {
        ko: "업로드할 파일을 드래그해주세요.",
        en: "Please drag the file you want to upload"
    }
    load_file: {
        ko: "파일 불러오기",
        en: "Load File"
    }
    allowed_extensions: {
        ko: "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다.",
        en: "Only jpg, .png, pdf, zip, word, excel, and pptx files can be uploaded."
    }
}
