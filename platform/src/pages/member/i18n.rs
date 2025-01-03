use crate::utils::context::Language;

pub struct MemberTranslate {
    pub organization_management: String,
    pub team_member_management: String,
    pub team_member_description: String,

    pub total: String,
    pub manager: String,
    pub public_opinion_manager: String,
    pub analyst: String,
    pub repeater: String,
    pub lecturer: String,

    pub add_team_member: String,
    pub name: String,
    pub group: String,
    pub role: String,
    pub project: String,

    pub remove_team_member: String,
    pub remove_team_member_li: String,

    pub remove_info: String,
    pub remove_warning: String,
    pub remove: String,
    pub cancel: String,

    pub necessary: String,
    pub enter_email_address: String,
    pub enter_email_address_hint: String,
    pub email_format_info: String,
    pub privacy: String,
    pub necessary_input: String,
    pub select_role: String,
    pub select_group: String,
    pub public_opinion: String,
    pub investigation: String,
    pub invite: String,
}

pub fn translate(lang: Language) -> MemberTranslate {
    match lang {
        Language::En => MemberTranslate {
            organization_management: "Organization Management".to_string(),
            team_member_management: "Team Member Management".to_string(),
            team_member_description: "The team member management page helps you manage team member information efficiently and understand roles and tasks at a glance.".to_string(),

            total: "Total".to_string(),
            manager: "Manager".to_string(),
            public_opinion_manager: "Public Opinion Manager".to_string(),
            analyst: "Analyst".to_string(),
            repeater: "Repeater".to_string(),
            lecturer: "Lecturer".to_string(),

            add_team_member: "Add Team Member".to_string(),
            name: "Name".to_string(),
            group: "Group".to_string(),
            role: "Role".to_string(),
            project: "Project".to_string(),

            remove_team_member: "Remove Team Member".to_string(),
            remove_team_member_li: "Remove Team Member".to_string(),

            remove_info: "Are you sure you want to delete it?".to_string(),
            remove_warning: "Deleted team members cannot be restored. Please check again before deleting.".to_string(),
            remove: "Remove".to_string(),
            cancel: "Cancel".to_string(),

            necessary: "*[Essential]".to_string(),
            enter_email_address: "Enter your email address".to_string(),
            enter_email_address_hint: "Enter your email address".to_string(),
            email_format_info: "Please enter the email format e.g voicekorea@company.com.".to_string(),
            privacy: "Privacy".to_string(),
            necessary_input: "Required input".to_string(),
            select_group: "Group selection".to_string(),
            select_role: "Role selection".to_string(),
            public_opinion: "Public Opinion".to_string(),
            investigation: "Investigation".to_string(),
            invite: "Invite".to_string(),
        },
        Language::Ko => MemberTranslate {
            organization_management: "조직관리".to_string(),
            team_member_management: "팀원 관리".to_string(),
            team_member_description: "팀원 관리 페이지는 팀원 정보를 효율적으로 관리하고 역할과 업무를 한눈에 파악할 수 있도록 도와줍니다.".to_string(),

            total: "전체".to_string(),
            manager: "관리자".to_string(),
            public_opinion_manager: "공론 관리자".to_string(),
            analyst: "분석가".to_string(),
            repeater: "중계자".to_string(),
            lecturer: "강연자".to_string(),

            add_team_member: "팀원 추가하기".to_string(),
            name: "이름".to_string(),
            group: "그룹".to_string(),
            role: "역할".to_string(),
            project: "프로젝트".to_string(),

            remove_team_member: "팀원 삭제".to_string(),
            remove_team_member_li: "팀원 삭제하기".to_string(),

            remove_info: "정말 삭제하시겠습니까?".to_string(),
            remove_warning: "삭제된 팀원은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.".to_string(),
            remove: "삭제하기".to_string(),
            cancel: "취소하기".to_string(),

            necessary: "*[필수]".to_string(),
            enter_email_address: "이메일 주소 입력하기".to_string(),
            enter_email_address_hint: "이메일 주소 입력".to_string(),
            email_format_info: "이메일 형식은 e.g voicekorea@company.com 으로 입력해주세요.".to_string(),
            privacy: "개인정보".to_string(),
            necessary_input: "필수 입력".to_string(),
            select_group: "그룹 선택".to_string(),
            select_role: "역할 선택".to_string(),
            public_opinion: "공론".to_string(),
            investigation: "조사".to_string(),
            invite: "초대하기".to_string(),
        },
    }
}
