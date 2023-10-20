import { lazy } from "react";

const Calendar = lazy(() => import("../pages/Calendar"));
const Chart = lazy(() => import("../pages/Chart"));
const FormElements = lazy(() => import("../pages/Form/FormElements"));
const FormLayout = lazy(() => import("../pages/Form/FormLayout"));
const Alerts = lazy(() => import("../pages/UiElements/Alerts"));
const Buttons = lazy(() => import("../pages/UiElements/Buttons"));

const coreRoutes = [
  {
    path: "/calendar",
    title: "Calender",
    component: Calendar,
  },
  {
    path: "/forms/form-elements",
    title: "Forms Elements",
    component: FormElements,
  },
  {
    path: "/forms/form-layout",
    title: "Form Layouts",
    component: FormLayout,
  },
  {
    path: "/chart",
    title: "Chart",
    component: Chart,
  },
  {
    path: "/ui/alerts",
    title: "Alerts",
    component: Alerts,
  },
  {
    path: "/ui/buttons",
    title: "Buttons",
    component: Buttons,
  },
];

const routes = [...coreRoutes];
export default routes;
