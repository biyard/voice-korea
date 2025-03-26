#![allow(unused_variables)]
use crate::response::AgeV3;
use crate::response::Attribute;
#[allow(unused)]
use crate::Result;
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

        let mut age_values = vec![];
        let mut gender_values = vec![];
        let mut region_values = vec![];
        let mut salary_values = vec![];

        for attribute in attributes {
            match attribute {
                Attribute::Age(age_v3) => {
                    age_values.push(age_v3);
                }
                Attribute::Gender(gender_v2) => {
                    gender_values.push(gender_v2);
                }
                Attribute::Region(region_v2) => {
                    region_values.push(region_v2);
                }
                Attribute::Salary(salary_v2) => {
                    salary_values.push(salary_v2);
                }
                Attribute::None => {}
            }
        }

        //NOTE: if value is none, it covers all attributes.
        if age_values.is_empty() {
            cover_age = true;
        }
        if gender_values.is_empty() {
            cover_gender = true;
        }
        if region_values.is_empty() {
            cover_region = true;
        }
        if salary_values.is_empty() {
            cover_salary = true;
        }

        for attr in other {
            match attr {
                Attribute::Age(age_attr) => {
                    if cover_age {
                        continue;
                    }

                    cover_age = age_values.iter().any(|panel_age| match age_attr {
                        AgeV3::Specific(target) => {
                            let (min, max) = panel_age.to_range();
                            *target >= min && *target <= max
                        }
                        AgeV3::Range {
                            inclusive_min,
                            inclusive_max,
                        } => {
                            let (min, max) = panel_age.to_range();
                            inclusive_min >= &min && inclusive_max <= &max
                        }
                        AgeV3::None => false,
                    });
                }

                Attribute::Gender(target) => {
                    if !cover_gender {
                        cover_gender = gender_values.contains(target);
                    }
                }

                Attribute::Region(target) => {
                    if !cover_region {
                        cover_region = region_values.contains(target);
                    }
                }

                Attribute::Salary(target) => {
                    if !cover_salary {
                        cover_salary = salary_values.contains(target);
                    }
                }
                Attribute::None => {}
            }
        }

        cover_age && cover_gender && cover_region && cover_salary
    }
}
