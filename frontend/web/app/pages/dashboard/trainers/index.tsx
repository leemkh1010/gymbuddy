import { Anchor } from "@mantine/core";
import { UIMatch } from "react-router";
import { Handle } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>Trainer</Anchor>;
  }
}

export default function Trainers() {
  return (
    <div>
      <h1>Trainers</h1>
    </div>
  );
}