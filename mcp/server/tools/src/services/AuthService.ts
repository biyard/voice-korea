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

  public static async loginUser() {
    try {
      let authToken = null;
      const isAuthProtected=true
      const { data, headers } = await makeApiCall(`/users`,isAuthProtected,
        {
          method: 'POST',
          data: {
            login: {
              email: 'boniface.ebuka@gmail.com', ///process.env.API_USERNAME, 
              password: stringToHex('0sir1.holysinner2'),///stringToHex(process.env.API_PASSWORD || "")
            }
          }
        })
      if (!data) {
        throw new Error('Invalid user login details')
      }

      const setCookieHeader = headers['set-cookie']?.[0]
      if (setCookieHeader) {
        authToken = setCookieHeader
      } else {
        throw new Error('No token cookie received')
      }

      this.authData = {
        token: authToken,
        ...data
      };

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