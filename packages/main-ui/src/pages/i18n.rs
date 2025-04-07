use dioxus_translate::translate;

translate! {
    LoginTranslate;

    login: {
        ko: "로그인",
        en: "Login"
    },
    email: {
        ko: "이메일",
        en: "Email"
    },
    password: {
        ko: "비밀번호",
        en: "Password"
    },
    find_email: {
        ko: "이메일 찾기",
        en: "Find Email"
    },
    reset_pw: {
        ko: "비밀번호재설정",
        en: "Reset Password"
    },
    create_account: {
        ko: "회원가입",
        en: "Create Account"
    },
    check_title: {
        ko: "확인하세요!",
        en: "Check it out!"
    },
    check_description_1: {
        ko: "- 비밀번호는 영문, 숫자, 특수기호 조합으로 8자 이상 구성되어야 합니다.",
        en: "- The password must consist of at least 8 characters and a combination of letters, numbers, and special symbols."
    },
    check_description_2: {
        ko: "- ID, 비밀번호를 분실, 5회 오류횟수 초과시에는 홈페이지 비밀번호 재설정 또는 고객센터(XXXX-XXXX)로 문의해 주세요.",
        en: "- If you lose your ID or password or exceed 5 errors, please reset your website password or contact customer service (XXXX-XXXX)."
    },

    not_matched_error: {
        ko: "이메일 또는 비밀번호를 잘못 입력하셨습니다.",
        en: "You entered your email or password incorrectly."
    },
    not_exists_user_error: {
        ko: "존재하지 않는 유저입니다.",
        en: "This user does not exist."
    },
    login_failed_error: {
        ko: "로그인을 실패하셨습니다. 잠시 후 다시 시도해보세요.",
        en: "You have failed to log in. Please try again later."
    },
}

translate! {
    RootLayoutTranslate;

    logout: {
        ko: "로그아웃",
        en: "Logout",
    },
    overview: {
        ko: "Overview",
        en: "Overview",
    },
    search_project: {
        ko: "프로젝트 검색",
        en: "Search Project",
    },
    import_project: {
        ko: "기존 프로젝트 가져오기",
        en: "Import Existing Project",
    },
    survey_management: {
        ko: "설문 관리",
        en: "Survey Management",
    },
    questionnaire_management: {
        ko: "설문지 관리",
        en: "Questionnaire Management",
    },
    question_bank: {
        ko: "질문 뱅크",
        en: "Question Bank",
    },
    property_management: {
        ko: "속성 관리",
        en: "Property Management",
    },
    property_status: {
        ko: "속성 현황",
        en: "Property Status",
    },
    user_settings: {
        ko: "사용자 설정",
        en: "User Settings",
    },
}
