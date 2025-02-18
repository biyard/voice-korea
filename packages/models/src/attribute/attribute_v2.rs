#![allow(unused_variables)]
#[allow(unused)]
use crate::Result;
#[cfg(feature = "server")]
use by_axum::aide;
use by_macros::ApiModel;
use dioxus_translate::Translate;
use serde::{Deserialize, Serialize};

// NOTE: AgeV2 should cover specific age and range of age.
//       If you need to change range of age, you must consider interaction with Noncelab.
#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum AgeV2 {
    #[default]
    None = 0,
    #[translate(ko = "17세 이하", en = "Under 17 years old")]
    Teenager = 1, //17세 이하
    #[translate(ko = "18~29세", en = "18-29 years old")]
    Twenty = 2, //18~29세
    #[translate(ko = "30대", en = "30-39 years old")]
    Thirty = 3, //30대
    #[translate(ko = "40대", en = "40-49 years old")]
    Fourty = 4, //40대
    #[translate(ko = "50대", en = "50-59 years old")]
    Fifty = 5, //50대
    #[translate(ko = "60대", en = "60-69 years old")]
    Sixty = 6, //60대
    #[translate(ko = "70대 이상", en = "Over 70s")]
    Over = 7, //70대 이상
}

impl AgeV2 {
    pub fn to_range(&self) -> (u8, u8) {
        match self {
            AgeV2::None => (0, 100),
            AgeV2::Teenager => (0, 17),
            AgeV2::Twenty => (18, 29),
            AgeV2::Thirty => (30, 39),
            AgeV2::Fourty => (40, 49),
            AgeV2::Fifty => (50, 59),
            AgeV2::Sixty => (60, 69),
            AgeV2::Over => (70, 100),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum GenderV2 {
    #[default]
    None = 0,
    #[translate(ko = "남성")]
    Male = 1,
    #[translate(ko = "여성")]
    Female = 2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum RegionV2 {
    #[default]
    None = 0,
    #[translate(ko = "서울")]
    Seoul = 2,
    #[translate(ko = "부산")]
    Busan = 51,
    #[translate(ko = "대구")]
    Daegu = 53,
    #[translate(ko = "인천")]
    Incheon = 32,
    #[translate(ko = "광주")]
    Gwangju = 62,
    #[translate(ko = "대전")]
    Daejeon = 42,
    #[translate(ko = "울산")]
    Ulsan = 52,
    #[translate(ko = "세종")]
    Sejong = 44,
    #[translate(ko = "경기")]
    Gyeonggi = 31,
    #[translate(ko = "강원")]
    Gangwon = 33,
    #[translate(ko = "충북")]
    Chungbuk = 43,
    #[translate(ko = "충남")]
    Chungnam = 41,
    #[translate(ko = "전북")]
    Jeonbuk = 63,
    #[translate(ko = "전남")]
    Jeonnam = 61,
    #[translate(ko = "경북")]
    Gyeongbuk = 54,
    #[translate(ko = "경남")]
    Gyeongnam = 55,
    #[translate(ko = "제주")]
    Jeju = 64,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case")]
pub enum SalaryV2 {
    #[default]
    None = 0,
    #[translate(ko = "2400만원 이하", en = "Less than 24 million won")]
    TierOne = 1,
    #[translate(ko = "2400만원 ~ 5000만원", en = "24 million won ~ 50 million won")]
    TierTwo = 2,
    #[translate(ko = "5000만원 ~ 8000만원", en = "50 million won ~ 80 million won")]
    TierThree = 3,
    #[translate(ko = "8000만원 ~ 10000만원", en = "80 million won ~ 100 million won")]
    TierFour = 4,
    #[translate(ko = "10000만원 이상", en = "More than 100 million won")]
    TierFive = 5,
}
