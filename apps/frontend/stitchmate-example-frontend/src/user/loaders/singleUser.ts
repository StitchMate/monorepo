import { ports } from "@stitchmate/user-application";
import { createResource } from "solid-js";

const loadSingleUser = (repository: ports.outboundPorts.UserRepository) => {
  return ({ params }: { params: any; location: any }) => {
    const [user] = createResource(
      () => params.id,
      async (id: string) => repository.get(id)
    );
    return user;
  };
};

export default loadSingleUser;