use dioxus_translate::translate;

translate! {
    CreateTranslate;

    authorization: {
        ko: "본인인증",
        en: "Identity Verification"
    },
    individual: {
        ko: "개인",
        en: "Individual"
    },
    company: {
        ko: "법인",
        en: "Corporation"
    },
    individual_description: {
        ko: "회원가입을 위한 본인확인 단계입니다. 만14세 미만인 경우 부모님(법정대리인)과 함께 진행하셔야 합니다. 인증수단을 미리 준비해주세요.",
        en: "This is the identity verification step for membership registration. If you are under 14 years of age, you must participate with your parents (legal representative). Please prepare your authentication method in advance."
    },
    phone: {
        ko: "휴대폰",
        en: "Cellphone"
    },
    phone_description: {
        ko: "본인 명의로 된 휴대폰으로 인증번호를 전송 받아 인증",
        en: "Authentication by receiving a verification number sent to a mobile phone in your name"
    },
    check_title: {
        ko: "확인하세요!",
        en: "Check it out!"
    },
    check_description_1: {
        ko: "- 본인인증은 본인명의로 개통된 휴대폰으로만 가능합니다.",
        en: "- Identity verification is only possible using a mobile phone activated in your name."
    },
    check_description_2: {
        ko: "- 본인인증 절차가 정상적으로 이루어지지 않을 경우 휴대폰 본인인증은 XXXX(XXXX-XXXX)로 문의하시기 바랍니다.",
        en: "- If the identity verification process does not work properly, please contact XXXX (XXXX-XXXX) for mobile phone identity verification."
    },
    check_description_3: {
        ko: "- 회원가입에 대한 다른 궁금한 사항은 고객센터(0000-0000)로 문의하여 주시기 바랍니다.",
        en: "- If you have any other questions about membership registration, please contact our customer service center (0000-0000)."
    },
    company_name: {
        ko: "회사명",
        en: "Company Name"
    },
    business_register_number: {
        ko: "사업자 등록번호",
        en: "Register Number"
    },
    company_name_example: {
        ko: "(주)바이야드",
        en: "Biyard Co"
    },
    business_register_number_example: {
        ko: "000-00-00000",
        en: "000-00-00000"
    },
    next: {
        ko: "다음",
        en: "Next"
    },
    agree_terms: {
        ko: "약관 동의",
        en: "Agree to Terms and Conditions"
    },
    agree_membership_terms: {
        ko: "회원약관 동의",
        en: "Agree to membership terms and conditions"
    },
    agree_privacy_policy: {
        ko: "개인정보처리방침",
        en: "Privacy policy"
    },
    entrust_personal_information: {
        ko: "개인정보처리의 위탁",
        en: "Entrustment of personal information processing"
    },
    essential: {
        ko: "(필수)",
        en: "(Essential)"
    },
    join_the_membership: {
        ko: "회원가입",
        en: "Join the Membership"
    },
    email_address: {
        ko: "이메일 주소",
        en: "Email Address"
    },
    send_authentication: {
        ko: "인증번호 전송",
        en: "Send Authentication Number"
    },
    authentication_number: {
        ko: "인증번호",
        en: "Authentication Number"
    },
    authentication_description_1: {
        ko: "- 3분 이내로 인증번호(6자리)를 입력해 주세요.",
        en: "- Please enter the authentication number (6 digits) within 3 minutes."
    },
    authentication_description_2: {
        ko: "- 인증시간이 초과된 경우 인증번호를 재발송 하신 후 입력해 주세요.",
        en: "- If the authentication time has expired, please resend the authentication number and then enter it."
    },
    password: {
        ko: "비밀번호 입력",
        en: "Input Password"
    },
    password_check: {
        ko: "비밀번호 확인",
        en: "Check Password"
    },
    company_info: {
        ko: "회사정보",
        en: "Company Information"
    },
    company_example: {
        ko: "주식회사 바이야드",
        en: "Biyard co"
    },
    name_info: {
        ko: "이름",
        en: "Name"
    },
    name_example: {
        ko: "OOO",
        en: "OOO"
    },
    phone_info: {
        ko: "휴대폰",
        en: "Cellphone"
    },
    phone_example: {
        ko: "000-0000-0000",
        en: "000-0000-0000"
    },
    address_info: {
        ko: "주소",
        en: "Address"
    },
    search_address: {
        ko: "주소검색",
        en: "Search Address"
    },
    check_membership_description_1: {
        ko: "- 법인으로 가입하시는 경우 대표메일을 이용해주세요.",
        en: "- If you are signing up as a corporation, please use the main email address."
    },
    check_membership_description_2: {
        ko: "- 받은 편지함에 이메일이 보이지 않는다면, 스팸함을 확인해주세요.",
        en: "- If you do not see the email in your inbox, please check your spam."
    },
    check_membership_description_3: {
        ko: "- 이메일이 정상적으로 받지 못했다면 고객센터(0000-0000)로 문의하여 주시기 바랍니다.",
        en: "- If you have not received the email properly, please contact customer service at 0000-0000."
    },
    complete_join_membership: {
        ko: "회원가입 완료",
        en: "Membership registration completed"
    },
    complete_join_membership_info: {
        ko: "회원가입 완료를 축하합니다.",
        en: "Congratulations on completing your membership registration."
    },
    complete: {
        ko: "완료",
        en: "Complete"
    },
    company_name_info: {
        ko: "법인명",
        en: "Corporation Name"
    },

    incollect_email_form: {
        ko: "이메일 형식이 올바르지 않습니다.",
        en: "The email format is incorrect."
    },
    input_password: {
        ko: "패스워드를 입력해주세요.",
        en: "Please enter your password."
    },
    incollect_two_password: {
        ko: "두 개의 비밀번호가 일치하지 않습니다.",
        en: "The two passwords do not match."
    },
    already_exists_user: {
        ko: "이미 존재하는 유저입니다.",
        en: "This user already exists."
    },
    incollect_authentication_number: {
        ko: "인증번호가 일치하지 않습니다.",
        en: "The authentication number does not match."
    },
    failed_store_data: {
        ko: "데이터 저장에 실패했습니다. 다시 시도해보세요.",
        en: "Failed to save data. Please try again."
    },
    invalid_password_pattern: {
        ko: "영문, 숫자, 특수기호 조합으로 8자 이상 구성해주세요.",
        en: "Please make up at least 8 characters using a combination of letters, numbers, and special symbols."
    },
    invalid_authorization_pattern: {
        ko: "인증번호 형태가 올바르지 않습니다.",
        en: "The authentication number format is incorrect."
    }
}
