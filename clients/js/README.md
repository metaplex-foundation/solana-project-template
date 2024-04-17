# JavaScript client for Mpl Project Name

A Umi-compatible JavaScript library for the project.

## Getting started

1. First, if you're not already using Umi, [follow these instructions to install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md).
2. Next, install this library using the package manager of your choice.
   ```sh
   npm install @metaplex-foundation/mpl-project-name
   ```
2. Finally, register the library with your Umi instance like so.
   ```ts
   import { mplProjectName } from '@metaplex-foundation/mpl-project-name';
   umi.use(mplProjectName());
   ```

You can learn more about this library's API by reading its generated [TypeDoc documentation](https://mpl-project-name-js-docs.vercel.app).

## Setting up Benchmarks
The GitHub workflow will automatically run benchmarks on pushes to the `main` branch, however it needs a gh-pages branch to deploy the hosted graph website to. Run the commands below to setup the gh-pages branch.
```sh
git checkout --orphan gh-pages
git reset --hard
git commit --allow-empty -m "Initializing gh-pages branch"
git push origin gh-pages
git checkout master
```

Afterwards, the webpage will be available at `https://<GITHUB_ORG_OR_USERNAME>.github.io/<REPO_NAME>/dev/bench/`

## Contributing

Check out the [Contributing Guide](./CONTRIBUTING.md) the learn more about how to contribute to this library.
