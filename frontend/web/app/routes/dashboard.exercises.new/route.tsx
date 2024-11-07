import { Anchor } from "@mantine/core";
import { UIMatch } from "@remix-run/react";
import { Handle } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>New</Anchor>;
  }
}

export default function ExercisesNew() {
  return (
    <div>
      <h1>New Exercise</h1>
    </div>
  );
}