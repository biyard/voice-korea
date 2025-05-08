use bdk::prelude::*;

translate! {
    VoteTranslate;

    organization_management: {
        ko: "조직 관리",
        en: "Organization Management"
    }
    deliberation_management: {
        ko: "공론 관리",
        en: "Deliberation Management"
    }
    start_deliberation: {
        ko: "공론 시작하기",
        en: "Start Deliberation"
    }
    vote_setting: {
        ko: "투표 설정",
        en: "Vote Setting"
    }

    vote: {
        ko: "투표",
        en: "Vote"
    }

    voting_items: {
        ko: "투표 항목",
        en: "Voting Item"
    }

    introduction_description: {
        ko: "공론의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다.",
        en: "Please explain the topic and purpose of the public discussion. This will make it easier for participants to understand and participate actively."
    }
}

translate! {
    QuestionListTranslate;

    input_title_hint: {
        ko: "제목을 입력해주세요.",
        en: "Please enter a title."
    }
    add_question: {
        ko: "새로운 질문을 추가해주세요.",
        en: "Please add a new question."
    }
}

translate! {
    FinalSurveyRewardTranslate;

    title: {
        ko: "예상 소요 시간 및 리워드",
        en: "Expected Time and Rewards"
    }

    description: {
        ko: "설문 응답에 걸리는 예상 소요 시간과 리워드를 입력해주세요. 입력된 시간은 리워드 지급과는 무관합니다.",
        en: "Please enter the estimated time it will take to complete the survey and the reward you wish to receive. The time you enter has no bearing on reward payment."
    }

    expected_time: {
        ko: "예상 소요 시간 (분)",
        en: "Estimated time (Minute)"
    }

    expected_time_hint: {
        ko: "소요 시간 입력 (단위: 초)",
        en: "Enter the time required (in seconds)"
    }

    expected_point: {
        ko: "응답 시 지급 포인트 입력",
        en: "Enter payment points when responding"
    }

    expected_point_hint: {
        ko: "포인트 입력",
        en: "Input point"
    }
}
