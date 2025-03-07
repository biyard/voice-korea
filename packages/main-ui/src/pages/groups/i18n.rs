use dioxus_translate::translate;

translate! {
    GroupTranslate;

    input_name_hint: {
        ko: "이름을 검색하세요.",
        en: "Please Search the Name",
    }
    organization_management: {
        ko: "조직관리",
        en: "Organization Management",
    },
    group_management: {
        ko: "그룹 관리",
        en: "Group Management",
    },
    create_group: {
        ko: "그룹 만들기",
        en: "Create Group",
    },
    group_description: {
        ko: "그룹 관리 페이지는 그룹 정보를 손쉽게 관리하고 구성과 상태를 한눈에 확인할 수 있습니다.",
        en: "The group management page allows you to easily manage group information and check its composition and status at a glance.",
    },
    group: {
        ko: "그룹",
        en: "Group",
    },
    personnel: {
        ko: "인원",
        en: "Personnel",
    },
    team_member: {
        ko: "팀원",
        en: "Team Member",
    },

    update_group_name: {
        ko: "그룹명 수정하기",
        en: "Update Group Name",
    },
    remove_group: {
        ko: "그룹 삭제",
        en: "Remove Group",
    },

    update_group_name_info: {
        ko: "그룹명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once the group name is modified, it cannot be undone.",
    },
    update_group_name_hint: {
        ko: "그룹명을 입력해주세요.",
        en: "Please enter the group name.",
    },
    update_group_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters.",
    },
    group_name: {
        ko: "그룹명",
        en: "Group Name"
    },
    update: {
        ko: "수정하기",
        en: "Edit",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },

    remove_warning: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?",
    },
    remove_info: {
        ko: "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다.",
        en: "Even if you delete a group, team members will remain, but you will need to set up the team members' groups again.",
    },
    remove: {
        ko: "삭제하기",
        en: "Delete",
    },

    input_contents: {
        ko: "내용 입력",
        en: "Input Contents"
    },
    create_group_hint: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    },
    add_team_member: {
        ko: "팀원 추가",
        en: "Add Team Member"
    },
    invite_project: {
        ko: "프로젝트 초대",
        en: "Invite Project"
    },
    public_opinion: {
        ko: "공론",
        en: "Public Opinion"
    },
    investigation: {
        ko: "조사",
        en: "Investigation"
    },
    create: {
        ko: "만들기",
        en: "Create"
    }
}

translate! {
    RemoveGroupModalTranslate;

    remove_warning: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?",
    },
    remove_info: {
        ko: "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다.",
        en: "Even if you delete a group, team members will remain, but you will need to set up the team members' groups again.",
    },
    remove: {
        ko: "삭제하기",
        en: "Delete",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}

translate! {
    UpdateGroupNameModalTranslate;

    update_group_name_info: {
        ko: "그룹명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once the group name is modified, it cannot be undone.",
    },
    update_group_name_hint: {
        ko: "그룹명을 입력해주세요.",
        en: "Please enter the group name.",
    },
    update_group_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters.",
    },
    group_name: {
        ko: "그룹명",
        en: "Group Name"
    },
    update: {
        ko: "수정하기",
        en: "Edit",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}
