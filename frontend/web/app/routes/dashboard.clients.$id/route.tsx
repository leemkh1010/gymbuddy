import { Anchor, Box, Button, Group, LoadingOverlay, Stack } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { LoaderFunctionArgs } from "@remix-run/node";
import { UIMatch, useLoaderData } from "@remix-run/react";
import React from "react";
import { Handle } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>{match.params.id}</Anchor>;
  }
}

export const loader = async ({
  params
}: LoaderFunctionArgs) => {
  return { id: params.id };
}

export default function ClientId() {
  const [isEditing, setIsEditing] = React.useState(false);
  const [loadingVisible, { toggle }] = useDisclosure(false);
  const { id } = useLoaderData<typeof loader>();

  return (
    <Stack gap="md">
      <Group justify="flex-end" align="end">
        <Button color="red">Delete</Button>
        <Button onClick={() => {
          setIsEditing(!isEditing)
          toggle()
        }}>{isEditing ? "Save" : "Edit"}</Button>
      </Group>
      <Box pos="relative">
        <LoadingOverlay visible={loadingVisible} zIndex={1000} overlayProps={{ radius: "sm", blur: 2 }} />
        <div>
          <h1>Client ID: {id}</h1>
        </div>
      </Box>
    </Stack>
  );
}