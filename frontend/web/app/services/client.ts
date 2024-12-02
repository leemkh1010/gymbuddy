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

export const get_client = async (id: string): Promise<Client> => {
  const req = await HttpClient.GET(`${API_BASE_URL}/clients/${id}`);

  const json = await req.json();

  return json.data as Client;
};

export const get_clients = async (): Promise<PaginatedResponse<Client>> => {
  const req = await HttpClient.GET(`${API_BASE_URL}/clients`);

  const json = await req.json();

  return json.data as PaginatedResponse<Client>;
};

export const create_client = async (client: Client): Promise<Client> => {
  const req = await HttpClient.POST(`${API_BASE_URL}/clients`, JSON.stringify(client));

  const json = await req.json();

  return json.data as Client;
};

export const update_client = async (client: Client): Promise<Client> => {
  const req = await HttpClient.PUT(`${API_BASE_URL}/clients/${client.id}`, JSON.stringify(client));

  const json = await req.json();

  return json.data as Client;
};

export const delete_client = async (id: string): Promise<void> => {
  await HttpClient.DELETE(`${API_BASE_URL}/clients/${id}`);
};