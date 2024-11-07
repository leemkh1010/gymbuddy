import { SerializeFrom } from "@remix-run/node";
import { UIMatch, useMatches } from "@remix-run/react";

export type Handle = {
  breadcrumb?: (match: UIMatch) => React.ReactNode;
}

export const useCustomMatches = <T>() => {
  const matches = useMatches() as UIMatch<T, Handle>[];
  return matches
}