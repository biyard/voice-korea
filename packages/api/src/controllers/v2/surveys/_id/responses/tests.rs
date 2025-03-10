use excel::SurveyResponseExcel;
use models::attribute_v2::{GenderV2, RegionV2, SalaryV2};

use crate::tests::{setup, TestContext};

use super::*;

#[tokio::test]
async fn test_survey_reponse() {
    let TestContext {
        user,
        now,
        endpoint,
        ..
    } = setup().await.unwrap();
    let org_id = user.orgs[0].id;

    let cli_panel = PanelV2::get_client(&endpoint);
    let mut panels = vec![];
    let mut panel_counts = vec![];

    let p = cli_panel
        .create(
            org_id,
            "group 1".to_string(),
            30,
            vec![
                Attribute::Age(AgeV3::Range {
                    inclusive_min: 50,
                    inclusive_max: 59,
                }),
                Attribute::Gender(GenderV2::Male),
                Attribute::Region(RegionV2::Busan),
                Attribute::Salary(SalaryV2::TierFive),
            ],
        )
        .await;

    assert!(p.is_ok(), "panel creation failed {:?}", p);
    let p = p.unwrap();

    panel_counts.push(PanelCountsV2 {
        created_at: p.created_at,
        updated_at: p.updated_at,

        panel_id: p.id,
        panel_survey_id: 0,
        user_count: 2,
    });
    panels.push(p);

    let p = cli_panel
        .create(
            org_id,
            "group 2".to_string(),
            50,
            vec![
                Attribute::Age(AgeV3::Range {
                    inclusive_min: 60,
                    inclusive_max: 69,
                }),
                Attribute::Gender(GenderV2::Female),
                Attribute::Region(RegionV2::Seoul),
                Attribute::Salary(SalaryV2::TierOne),
            ],
        )
        .await
        .unwrap();
    panel_counts.push(PanelCountsV2 {
        created_at: p.created_at,
        updated_at: p.updated_at,

        panel_id: p.id,
        panel_survey_id: 0,
        user_count: 2,
    });

    panels.push(p);

    let cli_survey = SurveyV2::get_client(&endpoint);
    let questions = vec![
        Question::SingleChoice(ChoiceQuestion {
            title: "single 1".to_string(),
            description: Some("test".to_string()),
            options: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }),
        Question::SingleChoice(ChoiceQuestion {
            title: "single 2".to_string(),
            description: Some("test".to_string()),
            options: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }),
        Question::ShortAnswer(SubjectiveQuestion {
            title: "short 3".to_string(),
            description: "test".to_string(),
        }),
        Question::Subjective(SubjectiveQuestion {
            title: "subjective 4".to_string(),
            description: "test".to_string(),
        }),
        Question::SingleChoice(ChoiceQuestion {
            title: "single 5".to_string(),
            description: Some("test".to_string()),
            options: vec!["a".to_string(), "b".to_string(), "c".to_string()],
        }),
    ];
    let survey = cli_survey
        .create(
            user.orgs[0].id,
            "name".to_string(),
            ProjectArea::City,
            now,
            now + 3600,
            "description".to_string(),
            100,
            questions,
            panels,
            panel_counts,
        )
        .await
        .unwrap();

    let cli_res = SurveyResponse::get_client(&endpoint);
    let survey_id = survey.id;

    let attributes1 = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 50,
            inclusive_max: 59,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Male),
        Attribute::Region(attribute_v2::RegionV2::Busan),
        Attribute::Salary(attribute_v2::SalaryV2::TierFive),
    ];

    let attributes2 = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 60,
            inclusive_max: 69,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Female),
        Attribute::Region(attribute_v2::RegionV2::Seoul),
        Attribute::Salary(attribute_v2::SalaryV2::TierOne),
    ];

    let proof = "user_proof".to_string();

    let _ = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes1.clone(),
            vec![
                Answer::SingleChoice { answer: 1 },
                Answer::SingleChoice { answer: 2 },
                Answer::ShortAnswer {
                    answer: "short answer".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective".to_string(),
                },
                Answer::SingleChoice { answer: 3 },
            ],
        )
        .await
        .unwrap();

    let _ = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes1.clone(),
            vec![
                Answer::SingleChoice { answer: 2 },
                Answer::SingleChoice { answer: 2 },
                Answer::ShortAnswer {
                    answer: "short answer".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective".to_string(),
                },
                Answer::SingleChoice { answer: 1 },
            ],
        )
        .await
        .unwrap();
    let res = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes1.clone(),
            vec![
                Answer::SingleChoice { answer: 2 },
                Answer::SingleChoice { answer: 2 },
                Answer::ShortAnswer {
                    answer: "short answer".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective".to_string(),
                },
                Answer::SingleChoice { answer: 1 },
            ],
        )
        .await;
    assert!(
        res.is_err(),
        "over quota response must be rejected {:?}",
        res
    );

    let _ = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes2.clone(),
            vec![
                Answer::SingleChoice { answer: 1 },
                Answer::SingleChoice { answer: 1 },
                Answer::ShortAnswer {
                    answer: "short answer 1".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective 1".to_string(),
                },
                Answer::SingleChoice { answer: 2 },
            ],
        )
        .await
        .unwrap();

    let incorrect_age_attribute = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 70,
            inclusive_max: 79,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Female),
        Attribute::Region(attribute_v2::RegionV2::Seoul),
        Attribute::Salary(attribute_v2::SalaryV2::TierOne),
    ];

    let incorrect_gender_attribute = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 60,
            inclusive_max: 69,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Male),
        Attribute::Region(attribute_v2::RegionV2::Seoul),
        Attribute::Salary(attribute_v2::SalaryV2::TierOne),
    ];

    let incorrect_region_attribute = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 60,
            inclusive_max: 69,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Female),
        Attribute::Region(attribute_v2::RegionV2::Busan),
        Attribute::Salary(attribute_v2::SalaryV2::TierOne),
    ];

    let incorrect_salary_attribute = vec![
        Attribute::Age(AgeV3::Range {
            inclusive_min: 60,
            inclusive_max: 69,
        }),
        Attribute::Gender(attribute_v2::GenderV2::Male),
        Attribute::Region(attribute_v2::RegionV2::Seoul),
        Attribute::Salary(attribute_v2::SalaryV2::TierTwo),
    ];

    for attributes in vec![
        incorrect_age_attribute,
        incorrect_gender_attribute,
        incorrect_region_attribute,
        incorrect_salary_attribute,
    ] {
        let res = cli_res
            .respond_answer(
                survey_id,
                proof.clone(),
                attributes.clone(),
                vec![
                    Answer::SingleChoice { answer: 3 },
                    Answer::SingleChoice { answer: 2 },
                    Answer::ShortAnswer {
                        answer: "short answer 3".to_string(),
                    },
                    Answer::Subjective {
                        answer: "subjective 3".to_string(),
                    },
                    Answer::SingleChoice { answer: 2 },
                ],
            )
            .await;
        assert!(
            res.is_err(),
            "incorrect attribute response must be rejected {:?} {:?}",
            attributes,
            res
        );
    }

    let res = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes2.clone(),
            vec![
                Answer::SingleChoice { answer: 3 },
                Answer::ShortAnswer {
                    answer: "short answer 3".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective 3".to_string(),
                },
                Answer::SingleChoice { answer: 2 },
            ],
        )
        .await;

    assert!(
        res.is_err(),
        "missing answer response must be rejected {:?}",
        res
    );

    let res = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes2.clone(),
            vec![
                Answer::SingleChoice { answer: 3 },
                Answer::ShortAnswer {
                    answer: "short answer 3".to_string(),
                },
                Answer::SingleChoice { answer: 2 },
                Answer::Subjective {
                    answer: "subjective 3".to_string(),
                },
                Answer::SingleChoice { answer: 2 },
            ],
        )
        .await;

    assert!(
        res.is_err(),
        "inconsistent answer response must be rejected {:?}",
        res
    );

    let q = cli_res
        .query(survey_id, SurveyResponseQuery::new(100))
        .await
        .unwrap();
    assert_eq!(q.total_count, 3);
    assert_eq!(q.items.len(), 3);

    let _ = cli_res
        .respond_answer(
            survey_id,
            proof.clone(),
            attributes2.clone(),
            vec![
                Answer::SingleChoice { answer: 3 },
                Answer::SingleChoice { answer: 2 },
                Answer::ShortAnswer {
                    answer: "short answer 3".to_string(),
                },
                Answer::Subjective {
                    answer: "subjective 3".to_string(),
                },
                Answer::SingleChoice { answer: 2 },
            ],
        )
        .await
        .unwrap();

    let q = cli_res
        .query(survey_id, SurveyResponseQuery::new(100))
        .await
        .unwrap();
    assert_eq!(q.total_count, 4);
    assert_eq!(q.items.len(), 4);

    #[cfg(feature = "full-test")]
    {
        let cli = SurveyResponseExcel::get_client(&endpoint);

        let res = cli.download_excel(org_id, survey_id).await.unwrap();

        assert!(!res.url.is_empty(), "excel download failed {:?}", res);
    }
}
