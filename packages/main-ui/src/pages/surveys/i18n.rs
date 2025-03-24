use dioxus_translate::translate;

translate! {
    QuestionListViewTranslate;

    input_title: {
        ko: "제목을 입력해주세요.",
        en: "Please enter a title."
    }
}

translate! {
    ErrorModalTranslate;

    title: {
        ko: "설문조사 확인",
        en: "Check the survey"
    }
    error_info: {
        ko: "빈 항목이 존재합니다. \n설문조사를 다시 확인해주세요.",
        en: "An empty entry exists. \nPlease check the survey again."
    }
    confirm: {
        ko: "확인",
        en: "Confirm"
    }
}

translate! {
    SettingPanelTranslate;

    composition_panel: {
        ko: "참여자 패널 구성",
        en: "Participant panel composition"
    }
    create_panel: {
        ko: "패널 새로 만들기",
        en: "Create new panel"
    }
    total_panel_setting: {
        ko: "전체 패널 설정",
        en: "Total Panel Setting"
    }
    total_panel_setting_description: {
        ko: "여론조사에 참여할 패널과 샘플링할 인원을 생성합니다.",
        en: "Create a panel to participate in the poll and the number of people to sample."
    }
    total_panel: {
        ko: "전체 패널",
        en: "Total Panel"
    }
    person: {
        ko: "명",
        en: "person"
    }
    select_panel: {
        ko: "패널 선택",
        en: "Select Panel"
    }
    total_people: {
        ko: "총 인원 수",
        en: "Total Number of People"
    }

    btn_complete: {
        ko: "완료하기",
        en: "Complete"
    }
    btn_temp_save: {
        ko: "임시저장",
        en: "Save as Draft"
    }
    btn_cancel: {
        ko: "취소하기",
        en: "Cancel"
    }
}

translate! {
    ObjectiveTranslate;

    option: {
        ko: "옵션",
        en: "Option"
    }
    add_option: {
        ko: "옵션 추가하기",
        en: "Add Option"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}

translate! {
    CreatePanelModalTranslate;

    panel_name_error: {
        ko: "패널명을 2글자 이상 입력해주세요.",
        en: "Please enter at least 2 characters for the panel name."
    }
    panel_count_error: {
        ko: "패널은 1명 이상으로 구성되어야 합니다.",
        en: "The panel must consist of at least 1 person."
    }
    age_error: {
        ko: "나이를 선택해주세요.",
        en: "Please select your age."
    }
    gender_error: {
        ko: "성별을 선택해주세요.",
        en: "Please select your gender."
    }
    region_error: {
        ko: "지역을 선택해주세요.",
        en: "Please select your region."
    }
    salary_error: {
        ko: "연봉을 선택해주세요.",
        en: "Please select your salary."
    }

    panel_name: {
        ko: "패널명",
        en: "Panel Name"
    }
    input_panel_name: {
        ko: "패널명 입력",
        en: "Input Panel Name"
    }
    input_panel_name_description: {
        ko: "중복 입력은 허용되지 않으며, 최소 2글자 이상 입력해야 합니다.",
        en: "Duplicate entries are not allowed, and you must enter at least 2 characters."
    }
    personnel: {
        ko: "인원",
        en: "Personnel"
    }
    save: {
        ko: "저장하기",
        en: "Save"
    }
    cancel: {
        ko: "취소",
        en: "Cancel"
    }

    age: {
        ko: "나이",
        en: "Age"
    }
    teenager: {
        ko: "17세 이하",
        en: "Under 17 years old"
    }
    twenty: {
        ko: "18~29세",
        en: "18-29 years old"
    }
    thirty: {
        ko: "30대",
        en: "30-39 years old"
    }
    fourty: {
        ko: "40대",
        en: "40-49 years old"
    }
    fifty: {
        ko: "50대",
        en: "50-59 years old"
    }
    sixty: {
        ko: "60대",
        en: "60-69 years old"
    }
    over: {
        ko: "70대 이상",
        en: "Over 70s"
    }

    gender: {
        ko: "성별",
        en: "Gender"
    }
    male: {
        ko: "남성",
        en: "Male"
    }
    female: {
        ko: "여성",
        en: "Female"
    }

    region: {
        ko: "지역",
        en: "Region"
    }
    seoul: {
        ko: "서울",
        en: "Seoul"
    }
    busan: {
        ko: "부산",
        en: "Busan"
    }
    daegu: {
        ko: "대구",
        en: "Daegu"
    }
    incheon: {
        ko: "인천",
        en: "Incheon"
    }
    gwangju: {
        ko: "광주",
        en: "Gwangju"
    }
    daejeon: {
        ko: "대전",
        en: "Daejeon"
    }
    ulsan: {
        ko: "울산",
        en: "Ulsan"
    }
    sejong: {
        ko: "세종",
        en: "Sejong"
    }
    gyeongi: {
        ko: "경기",
        en: "Gyeongi"
    }
    gangwon: {
        ko: "강원",
        en: "Gangwon"
    }
    chungbuk: {
        ko: "충북",
        en: "Chungbok"
    }
    chungnam: {
        ko: "충남",
        en: "Chungnam"
    }
    jeonbuk: {
        ko: "전북",
        en: "Jeonbuk"
    }
    jeonnam: {
        ko: "전남",
        en: "Jeonnam"
    }
    gyeonbuk: {
        ko: "경북",
        en: "Gyeonbuk"
    }
    gyeonnam: {
        ko: "경남",
        en: "Gyeonnam"
    }
    jeju: {
        ko: "제주",
        en: "Jeju"
    }

    salary: {
        ko: "연봉",
        en: "Salary"
    }
    tier_one: {
        ko: "2400만원 이하",
        en: "Less than 24 million won"
    }
    tier_two: {
        ko: "2400만원 ~ 5000만원",
        en: "24 million won ~ 50 million won"
    }
    tier_three: {
        ko: "5000만원 ~ 8000만원",
        en: "50 million won ~ 80 million won"
    }
    tier_four: {
        ko: "8000만원 ~ 10000만원",
        en: "80 million won ~ 100 million won"
    }
    tier_five: {
        ko: "10000만원 이상",
        en: "More than 100 million won"
    }
}

translate! {
    SubjectiveTranslate;

    input_title_hint: {
        ko: "제목을 입력해주세요",
        en: "Please enter a title"
    }
    input_description_hint: {
        ko: "내용을 입력해주세요",
        en: "Please enter a description"
    }
    remove: {
        ko: "삭제",
        en: "Remove"
    }
}

translate! {
    SurveyTranslate;

    survey_title: {
        ko: "조사 관리",
        en: "Survey Management"
    }
    survey_description: {
        ko: "조사관리 페이지는 다양한 조사 설계, 응답 수집, 결과 분석을 한 곳에서 효율적으로 관리할 수 있습니다.",
        en: "The survey management page allows you to efficiently manage various survey designs, response collection, and result analysis in one place."
    }
    search_hint: {
        ko: "검색어를 입력해주세요.",
        en: "Please enter your search term."
    }
    start_survey: {
        ko: "조사 시작하기",
        en: "Start Survey"
    }

    survey_type: {
        ko: "유형",
        en: "Type"
    }
    survey_field: {
        ko: "분야",
        en: "Field"
    }
    survey_project: {
        ko: "프로젝트",
        en: "Project"
    }
    survey_response_rate: {
        ko: "응답률",
        en: "Response Rate"
    }
    survey_panel: {
        ko: "패널",
        en: "Panel"
    }
    survey_period: {
        ko: "기간",
        en: "Period"
    }
    survey_status: {
        ko: "상태",
        en: "Status"
    }
    survey_view: {
        ko: "보기",
        en: "View"
    }

    detail_more: {
        ko: "자세히 보기",
        en: "Detail More"
    }
    view_results: {
        ko: "결과 보기",
        en: "View Results"
    }
    start_survey_create: {
        ko: "설문 시작하기",
        en: "Start Survey"
    }
    in_progress: {
        ko: "설문 진행 중",
        en: "In Progress"
    }

    update_survey: {
        ko: "조사 수정하기",
        en: "Update Survey"
    }
    remove_survey: {
        ko: "조사 삭제하기",
        en: "Remove Survey"
    }
    update_reward: {
        ko: "예상 소요 시간 및 리워드 수정하기",
        en: "Modify estimated time and rewards"
    }

    remove_modal_title: {
        ko: "설문 삭제",
        en: "Remove Survey"
    }
}

translate! {
    RemoveSurveyModalTranslate;

    remove_info: {
        ko: "정말 삭제하시겠습니까?",
        en: "Are you sure you want to delete it?",
    },
    remove_warning: {
        ko: "삭제한 설문은 복원할 수 없습니다. 삭제 전에 다시 한번 확인해주세요.",
        en: "Deleted surveys cannot be restored. Please check again before deleting.",
    },
    remove: {
        ko: "삭제하기",
        en: "Remove",
    },
    cancel: {
        ko: "취소하기",
        en: "Cancel",
    },
}

translate! {
    InputIntroductionTranslate;

    necessary_info: {
        ko: "조사 필수 정보",
        en: "Survey essential information"
    }
    input_introduction: {
        ko: "소개글 입력",
        en: "Input introduction"
    }
    introduction_description: {
        ko: "질문의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다.",
        en: "Please explain the topic and purpose of your question. Participants will be able to understand more easily and participate actively."
    }
    select_field: {
        ko: "분야 선택",
        en: "Select Field"
    }
    input_title_hint: {
        ko: "제목을 입력해주세요",
        en: "Please enter a title"
    }
    input_description_hint: {
        ko: "내용을 입력해주세요",
        en: "Please enter a description"
    }
}
