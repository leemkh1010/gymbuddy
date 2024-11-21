import { Button, Center, Fieldset, Group, Stack, TextInput } from "@mantine/core"
import { ActionFunctionArgs } from "@remix-run/node";
import { Form } from "@remix-run/react";

export const action = async ({
  request,
}: ActionFunctionArgs) => {
  const data = await request.formData();

  console.log(
    data
  )

  return null;
}

export default function Login() {
  return (
    <Center h="100vh" w="100vw">
      <Form method="POST" action="/login">
        <Fieldset legend="Login" w="90vw" maw="360"> 
          <Stack gap="md">
            <TextInput label="Email" placeholder="123@abc.com" />
            <TextInput label="Password" placeholder="xxxx" />
            <Group  justify="flex-end">
              <Button type="submit">Login</Button>
            </Group>
          </Stack>
        </Fieldset>
      </Form>
    </Center>
  );
}