use bdk::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct Controller {
    #[allow(dead_code)]
    lang: Language,
    #[allow(dead_code)]
    nav: Navigator,
}

impl Controller {
    #[allow(dead_code)]
    pub fn new(lang: Language) -> std::result::Result<Self, RenderError> {
        let ctrl = Self {
            lang,
            nav: use_navigator(),
        };

        Ok(ctrl)
    }

    // pub fn update_deliberation_info(&mut self, index: usize, opinion: StepCreateRequest) {
    //     let mut sequences = self.deliberation_sequences();
    //     sequences[index] = opinion;
    //     self.deliberation_sequences.set(sequences);
    // }

    // pub fn delete_deliberation_info(&mut self, index: usize) {
    //     let mut sequences = self.deliberation_sequences();
    //     sequences.remove(index);
    //     self.deliberation_sequences.set(sequences);
    // }

    // pub fn add_deliberation_info(&mut self) {
    //     let mut sequences = self.deliberation_sequences();
    //     sequences.push(StepCreateRequest {
    //         step_type: StepType::GeneralPost,
    //         name: "".to_string(),
    //         started_at: 0,
    //         ended_at: 0,
    //     });
    //     self.deliberation_sequences.set(sequences);
    // }

    // pub fn check_deliberation_info(&self) -> bool {
    //     let sequences = &self.deliberation_sequences();

    //     for sequence in sequences {
    //         if sequence.started_at == 0 || sequence.ended_at == 0 {
    //             return false;
    //         }

    //         if sequence.started_at > sequence.ended_at {
    //             return false;
    //         }
    //     }

    //     true
    // }

    // pub fn set_deliberation_sequences(&mut self, steps: Vec<StepCreateRequest>) {
    //     self.deliberation_sequences.set(steps);
    // }

    // pub fn change_deliberation_sequences(
    //     &self,
    //     req: DeliberationCreateRequest,
    // ) -> DeliberationCreateRequest {
    //     let mut req = req;
    //     req.steps = self.deliberation_sequences();

    //     let deliberation_time = self.get_deliberation_time(req.clone().steps);
    //     req.started_at = deliberation_time.0;
    //     req.ended_at = deliberation_time.1;

    //     req
    // }

    // pub fn get_deliberation_time(&self, steps: Vec<StepCreateRequest>) -> (i64, i64) {
    //     let started_at = steps.iter().map(|s| s.started_at).min().unwrap_or(0);
    //     let ended_at = steps.iter().map(|s| s.ended_at).max().unwrap_or(0);

    //     (started_at, ended_at)
    // }
}
