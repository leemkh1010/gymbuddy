import { Anchor, Button, Group, Stack, TextInput } from "@mantine/core";
import { Form, redirect, UIMatch } from "@remix-run/react";
import { Handle } from "~/utils/hook";
import { useForm } from '@mantine/form';
import { ActionFunctionArgs } from "@remix-run/node";
import { notifications } from "@mantine/notifications";
import { HttpClient } from "~/utils/http_client";
import { Client, create_client, update_client } from "~/services/client";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>New</Anchor>;
  }
}

export const action = async ({
  request
}: ActionFunctionArgs) => {
  const formData = await request.formData();
  
  const req = await create_client({
    first_name: formData.get('first_name'),
    last_name: formData.get('last_name'),
    email: formData.get('email'),
  } as Client);

  return redirect("/dashboard/clients");
}

export default function ClientsNew() {
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      first_name: '',
      last_name: '',
      email: '',
    },

    validate: {
      first_name: (value) => value.trim().length > 0 ? null : 'First name is required',
      last_name: (value) => value.trim().length > 0 ? null : 'Last name is required',
      email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
    },
  });

  return (
    <Stack gap="md">
      <form method="post" action="/dashboard/clients/new">
        <TextInput
          withAsterisk
          label="First name"
          name="first_name"
          key={form.key('first_name')}
          
          {...form.getInputProps('first_name')}
        />
        <TextInput
          withAsterisk
          label="Last name"
          name="last_name"
          key={form.key('last_name')}
          {...form.getInputProps('last_name')}
        />
        <TextInput
          withAsterisk
          label="Email"
          name="email"
          placeholder="your@email.com"
          key={form.key('email')}
          {...form.getInputProps('email')}
        />
        <Group justify="flex-end" mt="md">
          <Button type="submit">Submit</Button>
        </Group>
      </form>
    </Stack>
  );
}