#![allow(unused_variables)]
use crate::response::Attribute;
#[allow(unused)]
use crate::Result;
use crate::{
    attribute_v2::{GenderV2, RegionV2, SalaryV2},
    response::AgeV3,
};
use bdk::prelude::*;
use by_types::QueryResponse;

#[derive(validator::Validate)]
#[api_model(base = "/v2/organizations/:org-id/panels", table = panels, iter_type=QueryResponse)]
pub struct PanelV2 {
    #[api_model(summary, primary_key, action = delete, read_action = [get_panel, find_by_id])]
    pub id: i64,
    #[api_model(summary, auto = insert)]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, action = [create], action_by_id = update, query_action = search_by)]
    pub name: String,
    //TODO: remove this field with removal of ui dependency
    #[api_model(summary, action = [create], action_by_id = update)]
    pub user_count: u64,

    #[api_model(summary, action = [create], action_by_id = update, type = JSONB, version = v0.1, nullable)]
    #[serde(default)]
    pub attributes: Vec<Attribute>,

    // #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    // pub age: AgeV2,
    // #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    // pub gender: GenderV2,
    // #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    // pub region: RegionV2,
    // #[api_model(summary, action = [create], action_by_id = update, type = INTEGER, nullable)]
    // pub salary: SalaryV2,
    #[api_model(summary, many_to_one = organizations)]
    pub org_id: i64,
}

impl PartialEq<Vec<crate::survey::response::Attribute>> for PanelV2 {
    fn eq(&self, other: &Vec<crate::survey::response::Attribute>) -> bool {
        use crate::survey::response::Attribute;

        let attributes = self.attributes.clone();

        let mut cover_age = false;
        let mut cover_gender = false;
        let mut cover_region = false;
        let mut cover_salary = false;

        let mut age_value = None;
        let mut gender_value = None;
        let mut region_value = None;
        let mut salary_value = None;

        for attribute in attributes {
            match attribute {
                Attribute::Age(age_v3) => {
                    cover_age = age_v3 == AgeV3::None;
                    age_value = Some(age_v3);
                }
                Attribute::Gender(gender_v2) => {
                    cover_gender = gender_v2 == GenderV2::None;
                    gender_value = Some(gender_v2);
                }
                Attribute::Region(region_v2) => {
                    cover_region = region_v2 == RegionV2::None;
                    region_value = Some(region_v2);
                }
                Attribute::Salary(salary_v2) => {
                    cover_salary = salary_v2 == SalaryV2::None;
                    salary_value = Some(salary_v2);
                }
                Attribute::None => {}
            }
        }

        for attr in other {
            match attr {
                Attribute::Age(AgeV3::Specific(age)) => {
                    if let Some(age_v3) = age_value.as_ref() {
                        let (min, max) = age_v3.to_range();
                        if *age >= min && *age <= max {
                            cover_age = true;
                        }
                    }
                }
                Attribute::Age(AgeV3::Range {
                    inclusive_min,
                    inclusive_max,
                }) => {
                    if let Some(age_v3) = age_value.as_ref() {
                        let (min, max) = age_v3.to_range();
                        if *inclusive_min >= min && *inclusive_max <= max {
                            cover_age = true;
                        }
                    }
                }
                Attribute::Age(AgeV3::None) => {}
                Attribute::Gender(gender) => {
                    if let Some(gender_v2) = gender_value {
                        if *gender == gender_v2 {
                            cover_gender = true;
                        }
                    }
                }
                Attribute::Region(region) => {
                    if let Some(region_v2) = region_value {
                        if *region == region_v2 {
                            cover_region = true;
                        }
                    }
                }
                Attribute::Salary(salary) => {
                    if let Some(salary_v2) = salary_value {
                        if *salary == salary_v2 {
                            cover_salary = true;
                        }
                    }
                }
                Attribute::None => {}
            }
        }

        cover_age && cover_gender && cover_region && cover_salary
    }
}
