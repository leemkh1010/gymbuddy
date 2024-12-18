import { Stack, Button, TextInput } from "@mantine/core";
import { ActionFunctionArgs } from "react-router";
import { Form, redirect } from "react-router";

type ClientForm = {
  url: string;
  action: "create" | "update";
  method: "POST" | "PUT";
}

type Field = {
  type: "text" | "email" | "tel";
  name: string;
  label: string;
  description?: string;
  placeholder?: string;
}

const fields: Field[] = [
  {
    name: "name",
    type: "text",
    label: "Name",
    placeholder: "John Doe",
  },
  {
    name: "email",
    type: "email",
    label: "Email",
    placeholder: "",
  },
  {
    name: "phone",
    type: "tel",
    label: "Phone",
    placeholder: "",
  },
]

// export const action = async ({
//   request
// }: ActionFunctionArgs) => {
//   console.log(request)
//   return redirect("/dashboard/clients");
// }

export default function ClientForm(props: ClientForm) {
  return (
    <Form
      method={props.method}
      // action={props.url}
    >
      <Stack
        align="stretch"
        justify="center"
        gap="md"
      >
        {fields.map((field) => (
          <TextInput
            type={field.type}
            label={field.label}
            placeholder={field.placeholder}
            description={field.description}
            name={field.name}
          />
        ))}
        <Button type="submit">{props.action.toUpperCase()}</Button>
      </Stack>
    </Form>
  );
}