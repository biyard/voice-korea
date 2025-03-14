use dioxus_translate::Translate;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Translate)]
pub enum DeliberationStatus {
    #[translate(ko = "준비", en = "Ready")]
    Ready,
    #[translate(ko = "진행", en = "InProgress")]
    InProgress,
    #[translate(ko = "마감", en = "Finish")]
    Finish,
}
