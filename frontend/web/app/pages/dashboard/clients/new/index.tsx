import { Anchor, Button, Group, Stack, TextInput } from "@mantine/core";
import { redirect, UIMatch } from "react-router";
import { Handle } from "~/utils/hook";
import { useForm } from '@mantine/form';
import { ActionFunctionArgs } from "react-router";
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

  notifications.show({ title: 'Client created', message: 'Client has been created', color: 'teal' });

  // return redirect("/dashboard/clients");
  return null;
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
      first_name: (value) => value.trim().length && /^[A-z]$/.test(value) ? null : 'Required or contain invalid characters',
      last_name: (value) => value.trim().length && /^[A-z]$/.test(value) ? null : 'Required or contain invalid characters',
      email: (value) => value.trim().length && (/^\S+@\S+$/.test(value) ? null : 'Required or contain invalid characters'),
    },
  });

  return (
    <Stack gap="md">
      <form method="post" action="/dashboard/clients/new">
        <TextInput
          withAsterisk
          required
          label="First name"
          name="first_name"
          key={form.key('first_name')}
          
          {...form.getInputProps('first_name')}
        />
        <TextInput
          withAsterisk
          required
          label="Last name"
          name="last_name"
          key={form.key('last_name')}
          {...form.getInputProps('last_name')}
        />
        <TextInput
          withAsterisk
          label="Email"
          name="email"
          required
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