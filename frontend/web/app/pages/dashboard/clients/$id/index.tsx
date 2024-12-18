import { Anchor, Box, Button, Group, LoadingOverlay, Stack, TextInput } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { ActionFunctionArgs, LoaderFunctionArgs, redirect } from "react-router";
import { UIMatch, useLoaderData } from "react-router";
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
  const req = await get_client(params.id as string);

  const client = await req.json() as Client;

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

  return redirect('/dashboard/clients');
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
      first_name: (value) => value.trim().length && /^[A-z]$/.test(value) ? null : 'Required and only A-z characters',
      last_name: (value) => value.trim().length && /^[A-z]$/.test(value) ? null : 'Required and only A-z characters',
      email: (value) => value.trim().length && (/^\S+@\S+$/.test(value) ? null : 'Required or contain invalid form'),
    },
  });

  const onDeleteClick = () => {
  };

  const onEditClick = () => {
    setIsEditing(!isEditing);
  };

  const onSaveClick = () => {
    setIsEditing(!isEditing);
    toggle();
  };

  return (
    <Stack gap="md">
      <Box pos="relative">
        <LoadingOverlay visible={loadingVisible} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} />
        <Stack gap="md">
          <form method="PUT" action={`/dashboard/clients/${client.id}`}>
            <Group justify="flex-end" align="end">
              <Button type="submit" color="red" onClick={onDeleteClick}>Delete</Button>
              <Button  display={isEditing ? "none" : "block"} onClick={onEditClick}>Edit</Button>
              <Button type="submit" display={isEditing ? "block" : "none"} onClick={onSaveClick}>Save</Button>
            </Group>
            <input type="hidden" name="id" value={client.id} />
            <TextInput
              withAsterisk
              required
              readOnly={!isEditing}
              label="First name"
              name="first_name"
              key={form.key('first_name')}
              {...form.getInputProps('first_name')}
            />
            <TextInput
              withAsterisk
              required
              readOnly={!isEditing}
              label="Last name"
              name="last_name"
              key={form.key('last_name')}
              {...form.getInputProps('last_name')}
            />
            <TextInput
              withAsterisk
              readOnly
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