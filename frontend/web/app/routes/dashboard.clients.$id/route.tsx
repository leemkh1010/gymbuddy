import { Anchor, Box, Button, Group, LoadingOverlay, Stack, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "@remix-run/node";
import { UIMatch, useLoaderData } from "@remix-run/react";
import React from "react";
import { Handle } from "~/utils/hook";
import { Client } from "../dashboard.clients._index/route";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>{match.params.id}</Anchor>;
  }
}

export const loader = async ({
  params
}: LoaderFunctionArgs) => {
  const client = await fetch(`http://localhost:10000/api/user/${params.id}`);
  return { client: (await client.json()) as Client };
}

export const action = async ({
  request,
}: ActionFunctionArgs) => {
  const formData = await request.formData();

  await fetch(`http://localhost:10000/api/user/${formData.get('id')}`, {
    method: 'PUT',
    body: JSON.stringify({
      email: formData.get('email'),
      name: formData.get('name'),
      phone: formData.get('phone'),
    }),
    headers: {
      'Content-Type': 'application/json',
    },
  });

  return new Response(null, { status: 303, headers: { Location: '/dashboard/clients' } });
}

export default function ClientId() {
  const [isEditing, setIsEditing] = React.useState(false);
  const [loadingVisible, { toggle }] = useDisclosure(false);
  const { client } = useLoaderData<typeof loader>();

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: client.name,
      email: client.email,
      phone: client.phone,
    },

    validate: {
      name: (value) => value.trim().length > 0 ? null : 'Name is required',
      email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
      phone: (value) => (value.trim().length > 0 ? null : 'Phone is required'),
    },
  });

  return (
    <Stack gap="md">
      <Box pos="relative">
        {/* <LoadingOverlay visible={loadingVisible} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} /> */}
        <Stack gap="md">
          <form method="post" action={`/dashboard/clients/${client.id}`}>
            <Group justify="flex-end" align="end">
              <Button color="red">Delete</Button>
              <Button  display={isEditing ? "none" : "block"} onClick={() => {
                setIsEditing(!isEditing)
              }}>Edit</Button>
              <Button type="submit" display={isEditing ? "block" : "none"}>Save</Button>
            </Group>
            <input type="hidden" name="id" value={client.id} />
            <TextInput
              withAsterisk
              disabled={!isEditing}
              label="Email"
              name="email"
              placeholder="your@email.com"
              key={form.key('email')}
              {...form.getInputProps('email')}
            />
            <TextInput
              withAsterisk
              disabled={!isEditing}
              label="Name"
              name="name"
              placeholder="John Doe"
              key={form.key('name')}
              {...form.getInputProps('name')}
            />
            <TextInput
              withAsterisk
              disabled={!isEditing}
              label="Phone"
              name="phone"
              placeholder="123456"
              key={form.key('phone')}
              {...form.getInputProps('phone')}
            />
          </form>
        </Stack>
      </Box>
    </Stack>
  );
}