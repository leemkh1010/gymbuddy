import { Stack, Group, Anchor, Button, Drawer, TextInput, Select, Grid, ActionIcon, Table } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "react-router";
import { useLoaderData, Outlet, Form, redirect, UIMatch, useNavigate } from "react-router";
import CustomTable from "~/components/CustomTable";
import ClientForm from "./ClientForm";
import { IconSearch } from "@tabler/icons-react";
import { HttpClient } from "~/utils/http_client";
import { Client, get_clients } from "~/services/client";
import { PaginatedResponse } from "~/services/response";
import { Handle } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>Client</Anchor>;
  }
}

export const loader = async ({ params }: LoaderFunctionArgs) => {
  const req = await get_clients();

  const data = await req.json() as PaginatedResponse<Client>;

  return data;
};

const columns = [
  {
    key: "id",
    title: "Client ID",
  },
  {
    key: "first_name",
    title: "First Name",
  }, 
  {
    key: "last_name",
    title: "Last Name",
  },
  {
    key: "email",
    title: "Email",
  },
];

export default function Clients() {
  const { data: clients } = useLoaderData<typeof loader>();
  const navigate = useNavigate();

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
      {clients.map((client) => (
        <Table.Tr style={{ cursor: "pointer" }} onClick={() => navigate(`/dashboard/clients/${client.id}`)}>
          {columns.map((column) => (
            <Table.Td key={column.key}>
              {/* @ts-ignore */}
              {client[column.key]}
            </Table.Td>
          ))}
        </Table.Tr>
      ))}
    </CustomTable>
  </>;
}