declare module "app-env" {
  interface ENV {
    SM_USER_REPOSITORY_TYPE: string;
    SM_BASE_URL: string;
  }

  const appEnv: ENV;
  export default appEnv;
}
