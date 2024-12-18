import { ActionIcon, AppShell, Avatar, Breadcrumbs, Burger, Divider, Flex, Group, NavLink, Stack } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { Outlet } from 'react-router';
import { IconBell, IconSettings, IconTrain, IconUser, IconPhysotherapist } from '@tabler/icons-react';
import { useCustomMatches } from '~/utils/hook';

type Page = {
  title: string;
  link: string;
  icon: React.ReactNode;
}

const pages: Page[] = [
  {
    title: 'Exercises',
    link: '/dashboard/exercises',
    icon: <IconPhysotherapist />
  },
  { 
    title: 'Clients',
    link: '/dashboard/clients',
    icon: <IconUser />
  },
  {
    title: 'Trainers',
    link: '/dashboard/trainers',
    icon: <IconTrain />
  },
];

const footerPages: Page[] = [
  { title: 'Setting', link: '/dashboard/setting', icon: <IconSettings /> },
];

export default function Dashboard() {
  const [opened, { toggle }] = useDisclosure();
  const matches = useCustomMatches<null>();
  const links = matches.filter(
    match => match.handle && match.handle.breadcrumb
  ).map(match => match.handle.breadcrumb!(match));

  return (
    <AppShell
      header={{ height: 48 }}
      navbar={{
        width: 300,
        breakpoint: 'sm',
        collapsed: { mobile: !opened },
      }}
      padding="md"
    >
      <AppShell.Header>

        <Flex h="100%" pl="16pt" pr="16pt" direction="row" justify="space-between">
          <Group>
            <Burger
              opened={opened}
              onClick={toggle}
              hiddenFrom="sm"
              size="sm"
            />
          </Group>
          <Group>
            <ActionIcon variant="transparent">
              <IconBell color='grey' />
            </ActionIcon>
            <ActionIcon variant="transparent">
              <Avatar color='gray' variant="transparent" />
            </ActionIcon>
          </Group>
        </Flex>

      </AppShell.Header>

      <AppShell.Navbar p="md">
        <Flex
          style={{ height: '100%' }}
          direction="column"
          justify="space-between"
        >
          <AppShell.Section>
            {pages.map((page) => (
              <NavLink
                key={page.link}
                href={page.link}
                label={page.title}
                leftSection={page.icon}
              />
            ))}
          </AppShell.Section>
          <AppShell.Section>
            <Divider />
            {footerPages.map((page) => (
              <NavLink
                key={page.link}
                href={page.link}
                label={page.title}
                leftSection={page.icon}
              />
            ))}
          </AppShell.Section>
        </Flex>
      </AppShell.Navbar>

      <AppShell.Main>
        <Stack gap="md">
        <Group>
          <Breadcrumbs separator="→" separatorMargin="md" mt="xs">
            {links}
          </Breadcrumbs>
        </Group>
        <Outlet />
      </Stack>
      </AppShell.Main>
    </AppShell>
  );
}