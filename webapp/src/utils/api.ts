import type {AxiosError} from "axios";

export const dontRetryOn404 = (count: number, error: AxiosError<any>) => error.response?.status === 404 ? false : count < 2