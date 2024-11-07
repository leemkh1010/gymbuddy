import { LoaderFunctionArgs } from "@remix-run/node";

export default function Trainer({
  params
}: LoaderFunctionArgs) {
  return (
    <div>
      <h1>Trainer ID: {params.id}</h1>
    </div>
  );
}