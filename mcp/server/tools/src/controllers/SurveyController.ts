import { Container } from "typedi";
import SurveyService from "../services/SurveyService.js";

type ControllerFn = (args: any) => Promise<any>;

export const surveyController: Record<string, ControllerFn> = {
  get_surveys_in_a_project: async ({ id, question }) => {
    const surveyService = Container.get(SurveyService);
    return await surveyService.getProjectSurveys(id, question);
  },

  get_final_surveys_in_a_project: async ({ id, question }) => {
    const surveyService = Container.get(SurveyService);
    return await surveyService.getProjectFinalSurveys(id, question);
  },

  get_final_surveys_recommendation_in_a_project: async ({ id, question }) => {
    const surveyService = Container.get(SurveyService);
    return await surveyService.getProjectFinalSurveyRecommendation(id, question);
  },

  fetch_my_surveys: async ({ question }) => {
    const surveyService = Container.get(SurveyService);
    return await surveyService.fetchUserSurveys(question);
  },

  fetch_details_of_my_survey_by_survey_id: async ({ id, question }) => {
    const surveyService = Container.get(SurveyService);
    return await surveyService.fetchUserSurveyById(id, question);
  },
};
