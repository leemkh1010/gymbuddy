export type SuccessResponse<T> = {
  code: number;
  data: T;
}

export type ErrorResponse = {
  code: number;
  message: string;
  error?: string;
}

export type PaginatedResponse<T> = {
  data: T[],
  total: number,
  page: number,
  limit: number,
}