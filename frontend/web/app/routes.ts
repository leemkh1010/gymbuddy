import {
  type RouteConfig,
  route,
  index,
  layout,
  prefix,
} from "@react-router/dev/routes";

export default [
  // however your routes are defined
  index("./pages/index.tsx"),
  ...prefix('dashboard', [
    layout('./pages/dashboard/layout.tsx', [
      // index('./pages/dashboard/exercises/index.tsx'),
      ...prefix('exercises', [
        layout('./pages/dashboard/exercises/layout.tsx', [
          index('./pages/dashboard/exercises/index.tsx'),
          route(':id', './pages/dashboard/exercises/$id/index.tsx'),
          route('new', './pages/dashboard/exercises/new/index.tsx'),
        ]),
      ]),
      ...prefix('clients', [
        layout('./pages/dashboard/clients/layout.tsx', [
          index('./pages/dashboard/clients/index.tsx'),
          route(':id', './pages/dashboard/clients/$id/index.tsx'),
          route('new', './pages/dashboard/clients/new/index.tsx'),
        ]),
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