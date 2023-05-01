# Solana project template

A template for vanilla Solana programs and their clients

- Generate IDLs using [Shank](https://github.com/metaplex-foundation/shank)
- Generate clients using [Kinobi](https://github.com/metaplex-foundation/kinobi)
- Configure local validators using [Amman](https://github.com/metaplex-foundation/amman)

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

## Deploying JavaScript clients

TODO

- mention about secret variables
- mention vercel configs for docs
