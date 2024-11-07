import { Anchor, Box, Button, Group, LoadingOverlay, Stack, Tabs } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { LoaderFunctionArgs } from "@remix-run/node";
import { UIMatch, useLoaderData } from "@remix-run/react";
import React from "react";
import { Handle } from "~/utils/hook";
import { AreaChart } from '@mantine/charts';

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

enum GalleryType {
  Info = "info",
  Data = "data",
}

export const data = [
  {
    date: 'Mar 22',
    Apples: 2890,
    Oranges: 2338,
    Tomatoes: 2452,
  },
  {
    date: 'Mar 23',
    Apples: 2756,
    Oranges: 2103,
    Tomatoes: 2402,
  },
  {
    date: 'Mar 24',
    Apples: 3322,
    Oranges: 986,
    Tomatoes: 1821,
  },
  {
    date: 'Mar 25',
    Apples: 3470,
    Oranges: 2108,
    Tomatoes: 2809,
  },
  {
    date: 'Mar 26',
    Apples: 3129,
    Oranges: 1726,
    Tomatoes: 2290,
  },
];

export default function ExercisesId() {
  const [isEditing, setIsEditing] = React.useState(false);
  const [loadingVisible, { toggle }] = useDisclosure(false);
  const { id } = useLoaderData<typeof loader>();

  return (
    <Box pos="relative">
      <Tabs defaultValue={GalleryType.Info}>
        <Stack gap="md">
          <Tabs.List grow>
            <Tabs.Tab value={GalleryType.Info}>Information</Tabs.Tab>
            <Tabs.Tab value={GalleryType.Data}>Data</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value={GalleryType.Data}>
            <AreaChart
              h={300}
              data={data}
              dataKey="date"
              series={[
                { name: 'Apples', color: 'indigo.6' },
                { name: 'Oranges', color: 'blue.6' },
                { name: 'Tomatoes', color: 'teal.6' },
              ]}
              curveType="linear"
            />
          </Tabs.Panel>
          <Tabs.Panel value={GalleryType.Info}>
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
          </Tabs.Panel>
        </Stack>
      </Tabs>
    </Box>
  );
}