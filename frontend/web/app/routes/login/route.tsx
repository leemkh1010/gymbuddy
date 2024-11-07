import { Center, Fieldset, TextInput } from "@mantine/core"

export default function Login() {
  return (
    <Center h="100vh" w="100vw">
      <Fieldset legend="Login">
        <TextInput label="Email" placeholder="Your email" />
      </Fieldset>
    </Center>
  );
}