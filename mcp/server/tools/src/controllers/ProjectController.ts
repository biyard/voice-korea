import { Container } from "typedi";
import ProjectService from "../services/ProjectService.js";

type ControllerFn = (args: any) => Promise<any>;

export const projectController: Record<string, ControllerFn> = {
  get_project_by_id: async ({ id, question }) => {
    const service = Container.get(ProjectService);
    return await service.getProjectById(id, question);
  },

  search_projects_by_title: async ({ title, question }) => {
    const service = Container.get(ProjectService);
    return await service.searchProjects(title, question);
  },

  fetch_latest_projects: async ({ question }) => {
    const service = Container.get(ProjectService);
    return await service.fetchLatestProjects(question);
  },

  fetch_project_deliberations: async ({ id, question }) => {
    const service = Container.get(ProjectService);
    return await service.fetchProjectDeliberations(id, question);
  },

  fetch_project_discussions: async ({ id, question }) => {
    const service = Container.get(ProjectService);
    return await service.fetchProjectDiscussions(id, question);
  },
  
};
