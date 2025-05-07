import { Service } from 'typedi';
import { makeApiCall } from '../commons/utils/axios.js';
import { stringToHex } from '../commons/utils/index.js';

@Service()
export default class AuthService {
  private static authData: any | null = null;

  public static async getAuth(): Promise<any> {
    if (!this.authData) {
      await this.loginUser();
    }
    return this.authData!;
  }

  private static async loginUser()
        {
            try {
                const user = await makeApiCall(`/users`, 
                    { 
                        method: 'POST',
                        data: {
                            login:{
                              email: process.env.API_USERNAME, 
                              password: stringToHex(process.env.API_PASSWORD || "")}
                            }
                     })
                if (!user) {
                  return {
                    content: [{ type: "text", text: `Invalid user login details` }]
                  };
                }
        
                this.authData = user;

                return user;
            } catch (error: any) {
                return {
                  content: [{ type: "text", text: `Error trying to login user: ${error.message}` }]
                };
              }
        }

        public static logoutUser() {
          this.authData = null;
        }
        
}