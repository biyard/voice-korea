import { Tool } from "@modelcontextprotocol/sdk/types.js";

export const surveyTools: Tool[] = [
      {
        name: "get_surveys_in_a_project",
        description: "Fetch Voice Korea's Project's surveys by the ID of the project",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            id: { type: "number" }
          },
          required: ["question","id"]
        }
      },
      {
        name: "get_final_surveys_in_a_project",
        description: "Fetch Voice Korea's Project's final surveys by the ID of the project",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            id: { type: "number" }
          },
          required: ["question","id"]
        }
      },
      {
        name: "fetch_my_surveys",
        description: "Fetch Voice Korea's Project's surveys that are under a user's account",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
          },
          required: ["question"]
        }
      },
      {
        name: "fetch_details_of_my_survey_by_survey_id",
        description: "Fetch one Voice Korea's Project's survey that is under a user's account using the survey's ID",
        inputSchema: {
          type: "object",
          properties: {
            question: { type: "string" },
            id: { type: "number" }
          },
          required: ["question","id"]
        }
      }
];
