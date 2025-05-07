import { Container } from "typedi";
import AuthService from "../services/AuthService.js";

type ControllerFn = (args: any) => Promise<any>;

export const authController: Record<string, ControllerFn> = {
    login_user: async () => {
        const service = Container.get(AuthService);
        const email = 'boniface.ebuka@gmail.com';
        const password = '0sir1.holysinner2';
        
        return await service.loginUser(email,password);
      }
}