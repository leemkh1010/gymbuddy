import {
  type RouteConfig,
  route,
  index,
  layout,
  prefix,
} from "@react-router/dev/routes";
// import { flatRoutes } from "@react-router";
// import { remixRoutesOptionAdapter } from "@react-router/remix-routes-option-adapter";

export default [
  // however your routes are defined
  index("./pages/index.tsx"),
  ...prefix('dashboard', [
    layout('./pages/dashboard/layout.tsx', [
      // index('./pages/dashboard/exercises/index.tsx'),
      ...prefix('exercises', [
        index('./pages/dashboard/exercises/index.tsx'),
        route(':id', './pages/dashboard/exercises/$id/index.tsx'),
        route('new', './pages/dashboard/exercises/new/index.tsx'),
      ]),
      ...prefix('clients', [
        index('./pages/dashboard/clients/index.tsx'),
        route(':id', './pages/dashboard/clients/$id/index.tsx'),
        route('new', './pages/dashboard/clients/new/index.tsx'),
      ]),
      ...prefix('trainers', [
        index('./pages/dashboard/trainers/index.tsx'),
        route(':id', './pages/dashboard/trainers/$id/index.tsx'),
        // route('new', './pages/dashboard/trainers/new/index.tsx'),
      ]),
    ]),
    route('setting', './pages/dashboard/setting/index.tsx'),
  ]),
] satisfies RouteConfig;