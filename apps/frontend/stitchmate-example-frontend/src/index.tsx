/* @refresh reload */
import { QueryClient, QueryClientProvider } from "@tanstack/solid-query";
import { render } from "solid-js/web";
import { Router } from "@solidjs/router";
import UserRoutes from "./user/routes";

const queryClient = new QueryClient();

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error(
    "Root element not found. Did you forget to add it to your index.html? Or maybe the id attribute got misspelled?"
  );
}

const routes = [UserRoutes()];

render(
  () => (
    <QueryClientProvider client={queryClient}>
      <Router>{routes}</Router>
    </QueryClientProvider>
  ),
  root!
);
