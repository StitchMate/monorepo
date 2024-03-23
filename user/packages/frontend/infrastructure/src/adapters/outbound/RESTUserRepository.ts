import { Err, Ok, Result } from "@sniptt/monads";
import ky from "ky";
import { User } from "@stitchmate/user-domain";
import { ports } from "@stitchmate/user-application";

class RESTUserRepository implements ports.outboundPorts.UserRepository {
  private client = ky.create({});

  public constructor() {
    this.client = this.client.create({});
    return this;
  }

  public setBaseUrl(baseUrl: string) {
    this.client = this.client.extend({
      prefixUrl: baseUrl + "/user",
    });
  }

  async get(id: string): Promise<Result<User, Error>> {
    try {
      let result = await this.client.get(`${id}`);
      // TODO: Add Valibot schema on the domain instead...
      let data: User = await result.json();

      let user = new User({
        id: data.id,
        name: data.name,
        email: data.email,
        createdAt: new Date(data.createdAt),
        updatedAt: new Date(data.updatedAt),
      });

      return Ok(user);
    } catch (err: any) {
      const error = await err.response.json();
      return Promise.resolve(Err(error.message));
    }
  }
}

export default RESTUserRepository;
