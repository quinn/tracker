/**
 * Generated by orval v6.10.2 🍺
 * Do not edit manually.
 * tracker
 * OpenAPI spec version: 0.1.0
 */
import axios from 'axios'
import type {
  AxiosRequestConfig,
  AxiosResponse,
  AxiosError
} from 'axios'
import {
  useQuery
} from '@tanstack/react-query'
import type {
  UseQueryOptions,
  QueryFunction,
  UseQueryResult,
  QueryKey
} from '@tanstack/react-query'
import type {
  User
} from './model'


/**
 * Returns all users in the system.
 * @summary Get all users
 */
export const getAllUsers = (
     options?: AxiosRequestConfig
 ): Promise<AxiosResponse<User[]>> => {
    return axios.get(
      `/user`,options
    );
  }


export const getGetAllUsersQueryKey = () => [`/user`];

    
export type GetAllUsersQueryResult = NonNullable<Awaited<ReturnType<typeof getAllUsers>>>
export type GetAllUsersQueryError = AxiosError<unknown>

export const useGetAllUsers = <TData = Awaited<ReturnType<typeof getAllUsers>>, TError = AxiosError<unknown>>(
  options?: { query?:UseQueryOptions<Awaited<ReturnType<typeof getAllUsers>>, TError, TData>, axios?: AxiosRequestConfig}

  ):  UseQueryResult<TData, TError> & { queryKey: QueryKey } => {

  const {query: queryOptions, axios: axiosOptions} = options ?? {};

  const queryKey = queryOptions?.queryKey ?? getGetAllUsersQueryKey();

  

  const queryFn: QueryFunction<Awaited<ReturnType<typeof getAllUsers>>> = ({ signal }) => getAllUsers({ signal, ...axiosOptions });

  const query = useQuery<Awaited<ReturnType<typeof getAllUsers>>, TError, TData>(queryKey, queryFn, queryOptions) as  UseQueryResult<TData, TError> & { queryKey: QueryKey };

  query.queryKey = queryKey;

  return query;
}

