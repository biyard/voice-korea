use crate::{response::Answer, Question};

#[derive(Clone, PartialEq, Debug)]
pub enum ParsedQuestion {
    SingleChoice {
        answers: Vec<String>,
        response_count: Vec<i64>,
    },
    MultipleChoice {
        answers: Vec<String>,
        response_count: Vec<i64>,
    },
    ShortAnswer {
        answers: Vec<String>,
    },
    Subjective {
        answers: Vec<String>,
    },
}

impl From<(&Question, &Answer)> for ParsedQuestion {
    fn from((question, answer): (&Question, &Answer)) -> Self {
        match question {
            Question::SingleChoice(_) => {
                let options = question.options();
                let mut response_count = vec![0; options.len()];

                if let Answer::SingleChoice { answer } = answer {
                    response_count[(answer - 1) as usize] += 1;
                }

                ParsedQuestion::SingleChoice {
                    answers: options.clone(),
                    response_count,
                }
            }
            Question::MultipleChoice(_) => {
                let options = question.options();
                let mut response_count = vec![0; options.len()];

                if let Answer::MultipleChoice { answer } = answer {
                    for ans in answer {
                        response_count[(ans - 1) as usize] += 1;
                    }
                }

                ParsedQuestion::MultipleChoice {
                    answers: options.clone(),
                    response_count,
                }
            }
            Question::ShortAnswer(_) => {
                if let Answer::ShortAnswer { answer } = answer {
                    ParsedQuestion::ShortAnswer {
                        answers: vec![answer.clone()],
                    }
                } else {
                    ParsedQuestion::ShortAnswer { answers: vec![] }
                }
            }
            Question::Subjective(_) => {
                if let Answer::Subjective { answer } = answer {
                    ParsedQuestion::Subjective {
                        answers: vec![answer.clone()],
                    }
                } else {
                    ParsedQuestion::Subjective { answers: vec![] }
                }
            }
        }
    }
}

impl Default for ParsedQuestion {
    fn default() -> Self {
        ParsedQuestion::SingleChoice {
            answers: Vec::new(),
            response_count: Vec::new(),
        }
    }
}
