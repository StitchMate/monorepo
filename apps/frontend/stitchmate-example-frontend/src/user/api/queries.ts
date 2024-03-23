import { RepositoryType, UserRepositoryFactory } from "@stitchmate/user-infrastructure";
import { createQuery } from "@tanstack/solid-query";
import appEnv from "app-env";
import { userQueryKeys } from "./keys";

const userRepository = (() => {
    let repository = UserRepositoryFactory.createRepository(appEnv.SM_USER_REPOSITORY_TYPE as RepositoryType);
    repository.setBaseUrl(appEnv.SM_BASE_URL);
    return repository;
})();

export const useUserQuery = (id: string) => {
    return createQuery(() => ({
        queryKey: userQueryKeys.detail(id),
        queryFn: () => userRepository.get(id),
        enabled: !!id
    }))
}