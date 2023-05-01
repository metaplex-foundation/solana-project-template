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
3. Run the `init.sh` script to initialize the project. This will find/replace the variable above, rename some files/folders, update the README and, finally, remove the `init.sh` script.
   ```sh
   ./init.sh
   ```
4. [Read the `CONTRIBUTING.md` file](./CONTRIBUTING.md) to learn more about how to use the project.

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

However, the "Deploy JS Client" workflow — configured in `.github/workflows/deploy-js.yml` — requires a few more steps to works. See the [CONTRIBUTING.md file of the JavaScript client](./clients/js/CONTRIBUTING.md#setting-up-github-actions) for more information.
