import { Stack, Group, Anchor, Button, Drawer, TextInput, Select, Grid, ActionIcon } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "react-router";
import { useLoaderData, Outlet, Form, redirect, UIMatch } from "react-router";
import CustomTable from "~/components/CustomTable";
import { IconSearch } from "@tabler/icons-react";

export default function Clients() {
  return <>
      <Group align="end">
        <Select
          placeholder="Filter By"
          data={[]}
        />
        <TextInput
          placeholder="Search"
          rightSection={<ActionIcon variant="light" size="md"><IconSearch /></ActionIcon>}
        />
        <Anchor href="/dashboard/exercises/new">
          <Button>New</Button>
        </Anchor>
      </Group>
  </>;
}