import { Group, Breadcrumbs, Anchor, Stack } from "@mantine/core";
import { Outlet, UIMatch } from "react-router";
import { Handle, useCustomMatches } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>Client</Anchor>;
  }
}

export default function ClientsRoot() {
  const matches = useCustomMatches<null>();
  const links = matches.filter(
    match => match.handle && match.handle.breadcrumb
  ).map(match => match.handle.breadcrumb!(match));

  return <>
    <Stack gap="md">
      <Group>
        <Breadcrumbs separator="→" separatorMargin="md" mt="xs">
          {links}
        </Breadcrumbs>
      </Group>
      <Outlet />
    </Stack>
  </>;
}