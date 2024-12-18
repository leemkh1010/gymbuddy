import { Anchor, Box, Button, Group, LoadingOverlay, Stack, Tabs } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { LoaderFunctionArgs } from "react-router";
import { UIMatch, useLoaderData } from "react-router";
import React from "react";
import { Handle } from "~/utils/hook";
import { AreaChart } from '@mantine/charts';
import InformationPanel from "./InformationPanel";
import DataPanel from "./DataPanel";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>{match.params.id}</Anchor>;
  }
}

export type Loader = {
  id?: string;
}

export const loader = async ({
  params
}: LoaderFunctionArgs): Promise<Loader> => {
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
  const rootData = useLoaderData<typeof loader>();

  return (
    <Box pos="relative">
      <Tabs defaultValue={GalleryType.Info}>
        <Stack gap="md">
          <Tabs.List grow>
            <Tabs.Tab value={GalleryType.Info}>Information</Tabs.Tab>
            <Tabs.Tab value={GalleryType.Data}>Data</Tabs.Tab>
          </Tabs.List>
          <Tabs.Panel value={GalleryType.Info}>
            <InformationPanel />
          </Tabs.Panel>
          <Tabs.Panel value={GalleryType.Data}>
            <DataPanel
              rootData={rootData}
            />
          </Tabs.Panel>
        </Stack>
      </Tabs>
    </Box>
  );
}