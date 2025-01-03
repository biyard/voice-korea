use crate::utils::context::Language;

pub struct GroupTranslate {
    pub organization_management: String,
    pub group_management: String,
    pub group_description: String,
    pub create_group: String,
    pub group: String,
    pub personnel: String,
    pub team_member: String,

    pub update_group_name: String,
    pub remove_group: String,

    pub update_group_name_li: String,
    pub remove_group_li: String,

    pub update_group_name_info: String,
    pub update_group_name_hint: String,
    pub update_group_name_warning: String,
    pub update: String,
    pub cancel: String,

    pub remove_warning: String,
    pub remove_info: String,
    pub remove: String,
}

pub fn translate(lang: Language) -> GroupTranslate {
    match lang {
        Language::En => GroupTranslate {
            organization_management: "Organization Management".to_string(),
            group_management: "Group Management".to_string(),
            create_group: "Create Group".to_string(),
            group_description: "The group management page allows you to easily manage group information and check its composition and status at a glance.".to_string(),

            group: "Group".to_string(),
            personnel: "Personnel".to_string(),
            team_member: "Team Member".to_string(),

            update_group_name: "Update Group Name".to_string(),
            remove_group: "Remove Group".to_string(),

            update_group_name_li: "Update Group Name".to_string(),
            remove_group_li: "Remove Group".to_string(),

            update_group_name_info: "Once the group name is modified, it cannot be undone.".to_string(),
            update_group_name_hint: "Please enter the group name.".to_string(),
            update_group_name_warning: "Duplicate entries are not allowed, and you must enter at least 2 characters.".to_string(),
            update: "Edit".to_string(),
            cancel: "Cancel".to_string(),

            remove_warning: "Are you sure you want to delete it?".to_string(),
            remove_info: "Even if you delete a group, team members will remain, but you will need to set up the team members' groups again.".to_string(),
            remove: "Delete".to_string(),
        },
        Language::Ko => GroupTranslate {
            organization_management: "조직 관리".to_string(),
            group_management: "그룹 관리".to_string(),
            create_group: "그룹 만들기".to_string(),
            group_description: "그룹 관리 페이지는 그룹 정보를 손쉽게 관리하고 구성과 상태를 한눈에 확인할 수 있습니다.".to_string(),

            group: "그룹".to_string(),
            personnel: "인원".to_string(),
            team_member: "팀원".to_string(),

            update_group_name: "그룹명 수정하기".to_string(),
            remove_group: "그룹 삭제".to_string(),

            update_group_name_li: "그룹명 수정하기".to_string(),
            remove_group_li: "그룹 삭제하기".to_string(),

            update_group_name_info: "그룹명은 한 번 수정하면 되돌릴 수 없습니다.".to_string(),
            update_group_name_hint: "그룹명을 입력해주세요.".to_string(),
            update_group_name_warning: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.".to_string(),
            update: "수정하기".to_string(),
            cancel: "취소하기".to_string(),

            remove_warning: "정말 삭제하시겠습니까?".to_string(),
            remove_info: "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다.".to_string(),
            remove: "삭제하기".to_string(),
        },
    }
}
