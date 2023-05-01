# Solana project template

A template for vanilla Solana programs and their clients.

- Generate IDLs using [Shank](https://github.com/metaplex-foundation/shank)
- Generate clients for one or more programs using [Kinobi](https://github.com/metaplex-foundation/kinobi)
- Configure local validators using [Amman](https://github.com/metaplex-foundation/amman)
- Build, test and lint programs and clients using GitHub Actions.
- Deploy your [Umi](https://github.com/metaplex-foundation/umi) JavaScript client and its TypeScript documentation by dispatching a GitHub workflow.

## Getting started

1. [Use this template](https://github.com/metaplex-foundation/solana-project-template/generate) to create a new repository.
2. Open the `init.sh` script and update the following variables.
   ```sh
   NAME="mpl-project-name"
   DESCRIPTION="My project description"
   PUBLIC_KEY="MyProgram1111111111111111111111111111111111"
   ```
3. Run the `init.sh` script to initialize the project. This will find/replace the variable above, rename some files/folders and, finally, remove the `init.sh` script.

   ```sh
   ./init.sh
   ```

## Available commands

The root folder contains a private `package.json` containing a few scripts and JavaScript dependencies that help generate IDLs, clients and start a local validator. First, [ensure you have pnpm installed](https://pnpm.io/installation) and run the following command to install the dependencies.

```sh
pnpm install
```

Then, you can run the following commands.

- `pnpm generate:idls` - Generate IDLs for all programs, as configured in the `configs/shank.cjs` file.
- `pnpm generate:clients` - Generate clients using Kinobi, as configured in the `configs/kinobi.cjs` file.
- `pnpm generate` - Shortcut for `pnpm generate:idls && pnpm generate:clients`.
- `pnpm validator` - Start a local validator using Amman, as configured in the `configs/validator.cjs` file.
- `pnpm validator:localnet` - Same as `pnpm validator`.
- `pnpm validator:devnet` - Start a local validator using the `configs/validator.devnet.cjs` file.
- `pnpm validator:mainnet` - Start a local validator using the `configs/validator.mainnet.cjs` file.
- `pnpm validator:stop` - Stop the local validator.
- `pnpm validator:logs` - Show the logs of the local validator.

## Managing clients

Each client has its own README with instructions on how to get started. You can find them in the `clients` folder.

- [JavaScript client](./clients/js/README.md)

## Setting up CI/CD using GitHub actions

Most of the CI/CD should already be set up for you and the `.github/.env` file can be used to tweak the variables of the workflows.

However, the "Deploy JS Client" workflow — configured in `.github/workflows/deploy-js.yml` — requires a few more steps to works.

### Deploying JavaScript clients

In order to deploy JavaScript clients using GitHub actions, we need the following secret variables to be set up on the repository.

- `NPM_TOKEN` — An access token that can publish your packages to NPM.
- `VERCEL_TOKEN` — An access token that can deploy to Vercel.
- `VERCEL_ORG_ID` — The ID of the Vercel organization you want to deploy to.

Then, we'll need to create a new GitHub environment called `js-client-documentation` for the generated documentation of the JavaScript client. We can then select the `main` branch only and add the following secret variable to this specific environment.

- `VERCEL_PROJECT_ID` — The ID of the Vercel project you want to deploy to.
  The convention for Metaplex is to create a new Vercel project named `mpl-project-name-js-docs` with the following deployment settings:

  - Build Command: `pnpm run build:docs`
  - Output Directory: `docs`
  - Install Command: `pnpm install`
  - Development Command: _None_

With all that set up, you can now run the "Deploy JS Client" workflow by dispatching it from the GitHub UI.

![CleanShot 2023-05-01 at 12 16 58@2x](https://user-images.githubusercontent.com/3642397/235444901-6ee95f30-ed84-4eef-b1c4-8b8474ab82a4.png)
