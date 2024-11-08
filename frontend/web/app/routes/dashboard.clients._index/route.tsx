import { Stack, Group, Anchor, Button, Drawer, TextInput, Select, Grid, ActionIcon } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData, Outlet, Form, redirect, UIMatch } from "@remix-run/react";
import CustomTable from "~/components/CustomTable";
import ClientForm from "./ClientForm";
import { IconSearch } from "@tabler/icons-react";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  const users = await fetch('http://localhost:10000/api/users');

  const data = await users.json();

  return {
    clients: data.users as Client[],
  }
};

export type Client = {
  id: string;
  name: string;
  email: string;
  phone: string;
}

const columns = [
  {
    key: "id",
    title: "Client ID",
  },
  {
    key: "name",
    title: "Name",
  }, {
  key: "email",
  title: "Email",
}, {
  key: "phone",
  title: "Phone",
}];

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