import { Result } from "@sniptt/monads";
import { User } from "@stitchmate/user-domain";

export interface UserRepository {
    get(id: string): Promise<Result<User, Error>>
    setBaseUrl(baseUrl: string): void
}