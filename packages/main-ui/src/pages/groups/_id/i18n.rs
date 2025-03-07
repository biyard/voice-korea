use dioxus_translate::translate;

translate! {
    GroupDetailTranslate;

    update_group_name: {
        ko: "그룹명 수정하기",
        en: "Update Group Name",
    },
    remove_group: {
        ko: "그룹 삭제하기",
        en: "Remove Group"
    },
    remove_team_member: {
        ko: "팀원 삭제",
        en: "Remove Team Member"
    },
    add_team_member: {
        ko: "팀원 추가하기",
        en: "Add Team Member"
    },
    remove_project: {
        ko: "프로젝트 삭제",
        en: "Remove Project"
    }

    organization_management: {
        ko: "조직관리",
        en: "Organization Management",
    },
    group_management: {
        ko: "그룹 관리",
        en: "Group Management",
    },
    see_detail: {
        ko: "자세히 보기",
        en: "See Detail",
    },
    register_date: {
        ko: "등록된 날짜",
        en: "Register Date",
    },

    // GroupParticipantTranslate
    group_team_member: {
        ko: "그룹 팀원",
        en: "Group Team Member",
    },
    add_member: {
        ko: "팀원 추가하기",
        en: "Add Member",
    },
    name: {
        ko: "이름",
        en: "Name",
    },
    group: {
        ko: "그룹",
        en: "Group",
    },
    role: {
        ko: "역할",
        en: "Role",
    },
    project: {
        ko: "프로젝트",
        en: "Project",
    },
    no_group: {
        ko: "그룹 없음",
        en: "No Group"
    },
    no_role: {
        ko: "역할 없음",
        en: "No Role"
    },
    remove_team_member_li: {
        ko: "팀원 삭제하기",
        en: "Remove Team Member"
    }

    // CommonProjectTranslate;
    common_project: {
        ko: "공통 프로젝트",
        en: "Common Project",
    },
    add_project: {
        ko: "프로젝트 추가하기",
        en: "Create Project",
    },
    item: {
        ko: "항목",
        en: "Item",
    },
    panel: {
        ko: "패널",
        en: "Panel",
    },
    period: {
        ko: "기간",
        en: "Period",
    },
    status: {
        ko: "상태",
        en: "Status",
    },
    investigation: {
        ko: "조사",
        en: "Investigation"
    },
    public_opinion: {
        ko: "공론",
        en: "Public Opinion"
    },
    ready: {
        ko: "준비",
        en: "Ready"
    },
    in_progress: {
        ko: "진행",
        en: "In Progress"
    },
    finish: {
        ko: "마감",
        en: "Finish"
    },
    exclude_from_project: {
        ko: "프로젝트에서 제외하기",
        en: "Exclude From Project"
    },

    // RemoveProjectTranslate;
    remove_project_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    },
    remove_project_modal_info: {
        ko: "삭제된 프로젝트는 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted projects cannot be restored. Please check again before deleting."
    },
    remove: {
        ko: "삭제하기",
        en: "Remove"
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel"
    },

    // RemoveMemberTranslate
    remove_member_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    },
    remove_member_modal_info: {
        ko: "삭제된 팀원은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted team members cannot be restored. Please check again before deleting."
    },

    update_group_name_modal_info: {
        ko: "그룹명은 한 번 수정하면 되돌릴 수 없습니다.",
        en: "Once the group name is modified, it cannot be undone."
    },
    group_name: {
        ko: "그룹명",
        en: "Group Name"
    },
    update_group_name_hint: {
        ko: "그룹명을 입력해주세요.",
        en: "Please enter the group name."
    },
    update_group_name_warning: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    },
    update: {
        ko: "수정하기",
        en: "Update"
    },

    remove_group_modal_title: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?"
    },
    remove_group_modal_info: {
        ko: "그룹을 삭제해도 팀원들은 유지되지만, 팀원들의 그룹 설정을 다시 해야합니다.",
        en: "Even if you delete a group, team members will remain, but you will need to set up the team members' groups again."
    },

    necessary: {
        ko: "*[필수]",
        en: "*[essential]"
    },
    input_email_address: {
        ko: "이메일 주소 입력하기",
        en: "Enter your email address"
    },
    input_email_address_hint: {
        ko: "이메일 주소 입력",
        en: "Enter your email address"
    },
    input_email_address_info: {
        ko: "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요.",
        en: "Please enter the email format e.g voicekorea@company.com."
    },
    privacy: {
        ko: "개인정보",
        en: "Privacy"
    },
    required_input: {
        ko: "필수 입력",
        en: "Required Input"
    },
    select_role: {
        ko: "역할 선택",
        en: "Select Role"
    },
    invite_project: {
        ko: "프로젝트 초대",
        en: "Invite Project"
    },
    invite: {
        ko: "초대하기",
        en: "Invite"
    }

    manager: {
        ko: "관리자",
        en: "Manager"
    }
    public_opinion_manager: {
        ko: "공론 관리자",
        en: "Public Opinion Manager"
    }
    analyst: {
        ko: "분석가",
        en: "Analyst"
    }
    repeater: {
        ko: "중계자",
        en: "Mediator"
    }
    lecturer: {
        ko: "강연자",
        en: "Speaker"
    }
}
