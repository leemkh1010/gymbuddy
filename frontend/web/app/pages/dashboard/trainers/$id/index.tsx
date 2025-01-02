import { LoaderFunctionArgs } from "react-router";

export default function Trainer({
  params
}: LoaderFunctionArgs) {
  return (
    <div>
      <h1>Trainer ID: {params.id}</h1>
    </div>
  );
}