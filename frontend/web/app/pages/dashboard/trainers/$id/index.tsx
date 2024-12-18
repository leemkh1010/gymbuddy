import { Anchor } from "@mantine/core";
import { LoaderFunctionArgs, UIMatch } from "react-router";
import { Handle } from "~/utils/hook";

export const handle: Handle = {
  breadcrumb: (match: UIMatch) => {
    return <Anchor href={match.pathname}>{match.params.id}</Anchor>;
  }
}

export default function Trainer({
  params
}: LoaderFunctionArgs) {
  return (
    <div>
      <h1>Trainer ID: {params.id}</h1>
    </div>
  );
}