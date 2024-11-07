import { Stack, Group, Anchor, Button, Drawer, TextInput, Select, Grid, ActionIcon } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "@remix-run/node";
import { useLoaderData, Outlet, Form, redirect, UIMatch } from "@remix-run/react";
import CustomTable from "~/components/CustomTable";
import ClientForm from "./ClientForm";
import { IconSearch } from "@tabler/icons-react";

export const loader = async ({ params }: LoaderFunctionArgs) => {
  return {
    client: {
      id: params.id,
    }
  }
};

type Client = {
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
const data: Client[] = [
  {
    id: "1",
    name: "John Doe",
    email: "john@gmail.com",
    phone: "123456",
  },
  {
    id: "2",
    name: "Jane Doe",
    email: "",
    phone: "654321",
  },
  {
    id: "3",
    name: "John Smith",
    email: "",
    phone: "",
  },
]

export default function Clients() {
  return <>
      <Group align="end">
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
        data={data}
      >
        
      </CustomTable>
  </>;
}