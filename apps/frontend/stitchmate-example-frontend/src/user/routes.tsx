import { lazy } from "solid-js";
import { Route } from "@solidjs/router";

const UserRoutes = () => {
  return (
    <>
      <Route
        path="/users/:id"
        component={lazy(() => import("./pages/UserDetails"))}
      ></Route>
    </>
  );
};

export default UserRoutes;
