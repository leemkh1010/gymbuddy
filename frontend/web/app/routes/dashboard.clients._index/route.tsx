import { Stack, Group, Anchor, Button, Drawer, TextInput, Select, Grid, ActionIcon } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData, Outlet, Form, redirect, UIMatch } from "@remix-run/react";
import CustomTable from "~/components/CustomTable";
import ClientForm from "./ClientForm";
import { IconSearch } from "@tabler/icons-react";
import { HttpClient } from "~/utils/http_client";
import { Client, get_clients } from "~/services/client";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  const users = await HttpClient.GET('http://localhost:8080/api/v1/clients');

  const json = await users.json();

  const data = await get_clients();

  return {
    clients: json.data as Client[],
  }
};

const columns = [
  {
    key: "id",
    title: "Client ID",
  },
  {
    key: "name",
    title: "Name",
  }, 
  {
    key: "email",
    title: "Email",
  },
];

export default function Clients() {
  const { clients } = useLoaderData<typeof loader>();

  return <>
    <Group align="center">
      <Select
        placeholder="Filter By"
        data={['Email']}
      />

      <TextInput
        placeholder="Search"
        rightSection={<ActionIcon variant="light" size="md"><IconSearch /></ActionIcon>}
      />
      <Anchor href="/dashboard/clients/new">
        <Button>New</Button>
      </Anchor>
    </Group>

    <CustomTable
      columns={columns}
      data={clients}
    >

    </CustomTable>
  </>;
}