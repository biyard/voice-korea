import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';
import { CONFIGS } from '../commons/configs/index.js';
import AuthService from './AuthService.js';

@Service()
export default class SurveyService {
    public async getProjectSurveys(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/sample-surveys?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No surveys found with project ID ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching data project data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async getProjectFinalSurveys(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/final-surveys?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No final survey found with project ID ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching data project data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async getProjectFinalSurveyRecommendation(id: number, question: string)
    {
        try {
            const project = await makeApiCall(`/deliberations/${id}/drafts?param-type=read&action=get-by-id`, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No final survey recommendation found with project ID ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching data project data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    public async fetchUserSurveys(question: string)
    {
        try {
            const maxLimit = CONFIGS.MAX_LIMIT;
            const { token } = await AuthService.getAuth();
            const project = await makeApiCall(`/organizations/6/surveys?param-type=query&size=${maxLimit}&bookmark=1`, 
              { 
                method: 'GET',
                headers:{
                  Authorization: `Bearer ${token}`
                }
              })
            if (!project) {
              return {
                content: [{ type: "text", text: `Unable to find your surveys` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching user survey data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching your survey: ${error.message}` }]
            };
          }
    }

    public async fetchUserSurveyById(id: number, question: string)
    {
        try {
            const { token } = await AuthService.getAuth();

            const survey = await makeApiCall(`/organizations/6/surveys/${id}`, 
              { 
                method: 'GET',
                headers:{
                  Authorization: `Bearer ${token}`
                }
              })
            if (!survey) {
              return {
                content: [{ type: "text", text: `Unable to find the details of this your survey with ID: ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching user survey by ID's data is: ${JSON.stringify(survey, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching your survey: ${error.message}` }]
            };
          }
    }
}