import axios from 'axios'

import { CONFIGS } from '../configs/index.js'

const api = axios.create({
    baseURL: CONFIGS.BASE_API_URL,
    timeout: 10000,
    withCredentials: true
  })

  const AuthApi = axios.create({
    baseURL: CONFIGS.CONSOLE_AUTH_API_URL,
    timeout: 10000,
    withCredentials: true
  })

  export const makeApiCall = async (
    url: string,
    isAuthProtected: boolean,
    options: {
      method?: 'GET' | 'POST' | 'PUT' | 'DELETE' | 'PATCH'
      params?: Record<string, any>
      data?: any
      headers?: Record<string, string>
    } = {}
  ) => {
    try {
      let response;

      if (isAuthProtected) {
        response = await AuthApi({
          url,
          method: options.method || 'GET',
          params: options.params,
          data: options.data,
          headers: options.headers,
        })
      }else{
        response = await api({
          url,
          method: options.method || 'GET',
          params: options.params,
          data: options.data,
          headers: options.headers,
        })
      }

      return response
    } catch (error: any) {
      // console.error('API Error:', error)
      throw error?.message
    }
  }