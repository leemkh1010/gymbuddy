import { Anchor, Box, Button, Group, LoadingOverlay, Stack, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs } from "@remix-run/node";
import { UIMatch, useLoaderData } from "@remix-run/react";
import React from "react";
import { Handle } from "~/utils/hook";
import { Client, delete_client, get_client, update_client } from "~/services/client";
import { HttpClient } from "~/utils/http_client";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>{match.params.id}</Anchor>;
  }
}

export const loader = async ({
  params
}: LoaderFunctionArgs) => {
  const client = await get_client(params.id as string);
  return { client };
}

export const action = async ({
  request,
}: ActionFunctionArgs) => {
  const formData = await request.formData();

  if (request.method === 'DELETE') {
    const req = await delete_client(formData.get('id') as string);

    return new Response(null, { status: 200, headers: { Location: '/dashboard/clients' } });
  }

  const req = await update_client({
    id: formData.get('id') as string,
    first_name: formData.get('first_name') as string,
    last_name: formData.get('last_name') as string,
    email: formData.get('email') as string,
  } as Client);

  return new Response(null, { status: 200, headers: { Location: '/dashboard/clients' } });
}

export default function ClientId() {
  const [isEditing, setIsEditing] = React.useState(false);
  const [loadingVisible, { toggle }] = useDisclosure(false);
  const { client } = useLoaderData<typeof loader>();

  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      first_name: client.first_name,
      last_name: client.last_name,
      email: client.email,
    },

    validate: {
      first_name: (value) => value.trim().length > 0 ? null : 'First name is required',
      last_name: (value) => value.trim().length > 0 ? null : 'Last name is required',
      email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
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
              label="First name"
              name="first_name"
              key={form.key('first_name')}
              {...form.getInputProps('first_name')}
            />
            <TextInput
              withAsterisk
              disabled={!isEditing}
              label="Last name"
              name="last_name"
              key={form.key('last_name')}
              {...form.getInputProps('last_name')}
            />
            <TextInput
              withAsterisk
              disabled={!isEditing}
              label="Email"
              name="email"
              key={form.key('email')}
              {...form.getInputProps('email')}
            />
          </form>
        </Stack>
      </Box>
    </Stack>
  );
}