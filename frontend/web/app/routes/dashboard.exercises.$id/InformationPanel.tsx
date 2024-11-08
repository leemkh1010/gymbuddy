import { LoadingOverlay, Group, Button, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { useLoaderData } from "@remix-run/react";
import React from "react";
import { loader } from "./route";

export default function InformationPanel() {
  const { id } = useLoaderData<typeof loader>();
  const [isEditing, setIsEditing] = React.useState(false);
  const [loadingVisible, { toggle }] = useDisclosure(false);
  return <>
    <LoadingOverlay visible={loadingVisible} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} />
    <Group justify="flex-end" align="end">
      <Button color="red">Delete</Button>
      <Button onClick={() => {
        setIsEditing(!isEditing)
        toggle()
      }}>{isEditing ? "Save" : "Edit"}</Button>
    </Group>
    <div>
      <h1>Exercise ID: {id}</h1>
    </div>
  </>;
}