import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';
import { CONFIGS } from '../commons/configs/index.js';

@Service()
export default class ProjectService {
    constructor(
    ){}
    public async getProjectById(id: number, question: string)
    {
        try {
            const isAuthProtected = false
            const project = await makeApiCall(`/projects/${id}`, isAuthProtected, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No project found with ID ${id}` }]
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

    public async searchProjects(title: string, question: string)
    {
        try {
          const maxLimit = CONFIGS.MAX_LIMIT;
          const isAuthProtected = false;
            const {data} = await makeApiCall(`/projects?action=search&bookmark=1&size=${maxLimit}&title=${title}&param-type=query`,
              isAuthProtected,
               { method: 'GET' })
            const project = data;
            if (!project.items) {
              return {
                content: [{ type: "text", text: `No projects found with title ${title}` }]
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

    public async fetchLatestProjects(question: string)
    {
        try {
            const maxLimit = CONFIGS.MAX_LIMIT
            const isAuthProtected = false

            const {data} = await makeApiCall(`/projects?size=${maxLimit}&param-type=query`, isAuthProtected, { method: 'GET' })
            const project = data;
            
            if (!project.items) {
              return {
                content: [{ type: "text", text: `No projects found!` }]
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

    
    public async fetchProjectDeliberations(id: number, question: string)
    {
        try {
            const isAuthProtected = false
            const project = await makeApiCall(`/deliberations/${id}/contents?param-type=query&size=1`, isAuthProtected, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No project's deliberations found with project ID ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching project's deliberations' data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project: ${error.message}` }]
            };
          }
    }

    
    public async fetchProjectDiscussions(id: number, question: string)
    {
        try {
            const isAuthProtected = false
            const project = await makeApiCall(`/deliberations/${id}/ideas?param-type=query&size=1`, isAuthProtected, { method: 'GET' })
            if (!project) {
              return {
                content: [{ type: "text", text: `No project's discussions found with project ID ${id}` }]
              };
            }
    
            return {
              content: [
                {
                  type: "text",
                  text: `question asked is: ${question}, matching project's discussions' data is: ${JSON.stringify(project, null, 2)}`
                }
              ]
            };
        } catch (error: any) {
            return {
              content: [{ type: "text", text: `Error fetching project's discussions: ${error.message}` }]
            };
          }
    }
}