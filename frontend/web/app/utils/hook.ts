import { SerializeFrom } from "react-router";
import { UIMatch, useMatches } from "react-router";

export type Handle = {
  breadcrumb?: (match: UIMatch) => React.ReactNode;
}

export const useCustomMatches = <T>() => {
  const matches = useMatches() as UIMatch<T, Handle>[];
  return matches
}