import { Anchor, Button, Group, Stack, TextInput } from "@mantine/core";
import { Form, redirect, UIMatch } from "@remix-run/react";
import { Handle } from "~/utils/hook";
import { useForm } from '@mantine/form';
import { ActionFunctionArgs } from "@remix-run/node";
import { notifications } from "@mantine/notifications";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>New</Anchor>;
  }
}

export const action = async ({
  request
}: ActionFunctionArgs) => {
  const formData = await request.formData();

  await fetch('http://localhost:10000/api/user', {
    method: 'POST',
    body: JSON.stringify({
      email: formData.get('email'),
      name: formData.get('name'),
      phone: formData.get('phone'),
    }),
    headers: {
      'Content-Type': 'application/json',
    },
  })

  return redirect("/dashboard/clients");
} 

export default function ClientsNew() {
  const form = useForm({
    mode: 'uncontrolled',
    initialValues: {
      name: '',
      email: '',
      phone: '',
    },

    validate: {
      name: (value) => value.trim().length > 0 ? null : 'Name is required',
      email: (value) => (/^\S+@\S+$/.test(value) ? null : 'Invalid email'),
      phone: (value) => (value.trim().length > 0 ? null : 'Phone is required'),
    },
  });

  return (
    <Stack gap="md">
      <form method="post" action="/dashboard/clients/new">
        <TextInput
          withAsterisk
          label="Email"
          name="email"
          placeholder="your@email.com"
          key={form.key('email')}
          {...form.getInputProps('email')}
        />
        <TextInput
          withAsterisk
          label="Name"
          name="name"
          placeholder="John Doe"
          key={form.key('name')}
          {...form.getInputProps('name')}
        />
        <TextInput
          withAsterisk
          label="Phone"
          name="phone"
          placeholder="123456"
          key={form.key('phone')}
          {...form.getInputProps('phone')}
        />
        <Group justify="flex-end" mt="md">
          <Button type="submit">Submit</Button>
        </Group>
      </form>
    </Stack>
  );
}