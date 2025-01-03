use crate::utils::context::Language;

pub struct GroupTranslate {
    pub organization_management: String,
    pub group_management: String,
    pub create_group: String,
    pub group: String,
    pub personnel: String,
    pub team_member: String,
}

pub fn translate(lang: Language) -> GroupTranslate {
    match lang {
        Language::En => GroupTranslate {
            organization_management: "Organization Management".to_string(),
            group_management: "Group Management".to_string(),
            create_group: "Create Group".to_string(),

            group: "Group".to_string(),
            personnel: "Personnel".to_string(),
            team_member: "Team Member".to_string(),
        },
        Language::Ko => GroupTranslate {
            organization_management: "조직 관리".to_string(),
            group_management: "그룹 관리".to_string(),
            create_group: "그룹 만들기".to_string(),

            group: "그룹".to_string(),
            personnel: "인원".to_string(),
            team_member: "팀원".to_string(),
        },
    }
}
