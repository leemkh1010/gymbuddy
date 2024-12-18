import { API_BASE_URL } from "~/utils/env";
import { HttpClient } from "~/utils/http_client";
import { PaginatedResponse } from "./response";

export type Client = {
  id: string;
  first_name: string;
  last_name: string;
  email: string;
  created_at: number;
  updated_at: number;
}

export const get_client = async (id: string): Promise<Response> => {
  return await HttpClient.GET(`${API_BASE_URL}/clients/${id}`);
};

export const get_clients = async (): Promise<Response> => {
  return await HttpClient.GET(`${API_BASE_URL}/clients`);
};

export const create_client = async (client: Client): Promise<Response> => {
  return await HttpClient.POST(`${API_BASE_URL}/clients`, JSON.stringify(client));
};

export const update_client = async (client: Client): Promise<Response> => {
  return await HttpClient.PUT(`${API_BASE_URL}/clients/${client.id}`, JSON.stringify(client));
};

export const delete_client = async (id: string): Promise<Response> => {
  return await HttpClient.DELETE(`${API_BASE_URL}/clients/${id}`);
};