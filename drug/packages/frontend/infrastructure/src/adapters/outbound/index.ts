import RESTUserRepository from "./RESTUserRepository";
import { ports } from "@stitchmate/user-application";

export type RepositoryType = "http";
export type Config = {
    http?: {
        baseUrl: string
    }
}

class UserRepositoryFactory {
    static createRepository(repositoryType: RepositoryType): ports.outboundPorts.UserRepository {
        switch (repositoryType) {
            case 'http':
                return new RESTUserRepository();
        }
    }
}

export { 
    UserRepositoryFactory
}