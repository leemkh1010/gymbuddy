import type { MetaFunction } from "@remix-run/node";

import { AppShell, Burger, Center, Container, Grid, Loader, Stack, Text, Title } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { Link } from "@remix-run/react";

export const meta: MetaFunction = () => {
  return [
    { title: "Pose Analyser" },
    { name: "description", content: "Present" },
  ];
};

export default function Index({ children }: { children: React.ReactNode }) {
  // TODO: Check for token and do redirect
  return (
    <Center h="100vh" w="100vw" >
      <Stack align="center">
        <Title order={2}>Welcome</Title>
        <Text>We are verifying your identity, please wait...</Text>
        <Loader />
      </Stack>
    </Center>
  );
}