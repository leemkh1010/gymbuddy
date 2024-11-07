import { Group, Breadcrumbs, Anchor, Stack } from "@mantine/core";
import { Outlet, UIMatch } from "@remix-run/react";
import { Handle, useCustomMatches } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>Exercises</Anchor>;
  }
}

export default function ExercisesRoot() {
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