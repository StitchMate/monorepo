import("@stitchmate/user-infrastructure").catch((err) => {});
import { Match, Switch } from "solid-js";
import { useUserQuery } from "../api/queries";
import { useParams } from "@solidjs/router";

function UserDetails() {
  const params = useParams();
  const state = useUserQuery(params.id);

  return (
    <div>
      <Switch>
        <Match when={!params.id || state.status === 'pending'}>
          Loading...
        </Match>
        <Match when={state.error instanceof Error}>
          <span>Error: {(state.error as Error).message}</span>
        </Match>
        <Match when={state.data !== undefined && state.data.isErr()}>
          <span>Error: {JSON.stringify(state.data?.unwrapErr())}</span>
        </Match>
        <Match when={state.data !== undefined && state.data.isOk()}>
          <>
            <sm-user-card prop:user={state.data?.unwrap()}></sm-user-card>
          </>
        </Match>
      </Switch>
    </div>
  );
}

export default UserDetails;
