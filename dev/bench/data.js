window.BENCHMARK_DATA = {
  "lastUpdate": 1794344236257,
  "repoUrl": "https://github.com/metaplex-foundation/solana-project-template",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "90809591+blockiosaurus@users.noreply.github.com",
            "name": "blockiosaurus",
            "username": "blockiosaurus"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "758ba0725c7fdab63b4be629753bb4ccfb0134fc",
          "message": "ci: use the shared reusable workflows from metaplex-foundation/actions (#17)\n\n* ci: call the shared reusable workflows from metaplex-foundation/actions\n\nThe template is the reference caller: main.yml and the dispatch workflows\ncall the reusable workflows pinned by commit, and the repository-specific\nfacts live in .github/.env, .github/programs.json and file-filters.yml.\ninit.sh keeps working since the program name and public key appear in\nprograms.json and the workflow inputs as plain text.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* chore: refresh the root lockfile for @types/node\n\nThe lock has been out of date with package.json since the Kinobi 2.0 bump,\nso pnpm install --frozen-lockfile fails in CI.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions@7afc538992df\n\nPicks up the solana-verify version default in the shared workflows.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: generate IDLs on Rust 1.85 so shank-cli 0.4.5 compiles\n\nshank-js installs the newest shank-cli matching its range, and 0.4.5 requires\nthe edition2024 Cargo feature. The program toolchain stays on 1.83.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions@772e5a7cf740 and tighten the dispatch gates\n\n- deploy and publish only accept skipped build/test jobs when git_ref is a\n  release tag; a failed build or tag check now blocks them\n- check_tag rejects release tags that do not belong to this repository (or,\n  for deploy, to the selected program)\n- one deploy per program and cluster, one publish and one release at a time\n- the shared deploy workflow now emits its squads plan output and reads\n  programs.json from the dispatched revision, so older release tags deploy\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: shank-cli 0.4.5 needs rustc 1.88 for IDL generation\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions@089e761576b6\n\nPicks up the benchmark fix for repos without a gh-pages branch.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions@8f4db07ffb2a; check_tag confirms the tag exists\n\nPicks up the second CodeRabbit round on the shared workflows: programs.json\nand host tool pins read from the dispatched revision, publish from the tested\ncommit, resumable releases. check_tag now rejects a branch named like a\nrelease tag, and create_release waits for the program build.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions@da62cf9659ef\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions 6064713 (CodeRabbit round 3)\n\nDispatch workflows resolve git_ref to one commit in check_tag and pass\nthat commit to every build, test, publish, release and deploy job, so a\nmoving branch or a branch named like a tag cannot split a run across\ndifferent source. The release workflows get the merged branch name to\npick the released version.\nThe JS client CONTRIBUTING guide describes npm trusted publishing instead\nof NPM_TOKEN.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to actions 782403d\n\ndetect-changes works on pushes to private repositories again and\ncreate-release continues the program's version across tag families.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows at 759f332, trusted-publisher setup notes\n\n- Regenerated from metaplex-foundation/actions@759f332: filter-matrix\n  injection fix, publish reruns recover after a published-but-unreleased\n  run, create-release's check_tag drops its unreachable release branch.\n- CONTRIBUTING: npm trusted publisher takes the workflow filename alone,\n  must allow direct `npm publish`, and the release PR needs \"Allow GitHub\n  Actions to create and approve pull requests\".\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows at 7f55495\n\n- Regenerated from metaplex-foundation/actions@7f55495: publish reruns only\n  recover when the registry holds this commit's content, buffer-only Squads\n  deploys export the verified-build PDA transaction, a release whose asset\n  upload fails now fails, release workflows accept SemVer build metadata.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows at v2.0.0\n\n- Pin metaplex-foundation/actions at v2.0.0 (4ee4bcf), the merge of\n  metaplex-foundation/actions#18. Also brings rerun recovery that compares\n  unpacked crates, a non-fatal PDA export for buffer-only Squads deploys,\n  and a node_modules cache keyed by runner image.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n* ci: pin shared workflows at v2.0.1\n\n- Pin metaplex-foundation/actions at v2.0.1 (8fcadb6), the merge of\n  metaplex-foundation/actions#20: verify-from-repo fails on a hash\n  mismatch, no remote verification job on devnet or for private\n  repositories, npm provenance only from public repositories, and publish\n  reruns check the requested dist-tag.\n\nCo-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Fable 5.1 <noreply@anthropic.com>",
          "timestamp": "2026-11-03T16:30:46-05:00",
          "tree_id": "63b2889e37e9c4da2155ae239285c4bb4fb06899",
          "url": "https://github.com/metaplex-foundation/solana-project-template/commit/758ba0725c7fdab63b4be629753bb4ccfb0134fc"
        },
        "date": 1793742030574,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "CU: create a new account",
            "value": 3734,
            "unit": "Compute Units"
          },
          {
            "name": "Space: create a new account",
            "value": 39,
            "unit": "Bytes"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "90809591+blockiosaurus@users.noreply.github.com",
            "name": "blockiosaurus",
            "username": "blockiosaurus"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "50fcbe04840003d7dfc113218ffa8c3fb258ba0d",
          "message": "ci: faster shared workflows (actions v2.1.1), Node 24 (#19)\n\n* ci: faster shared workflows (actions v2.1.0 candidate), Node 24\n\nRegenerated from the shared-CI spec against metaplex-foundation/actions\nci/faster-ci: generate_clients runs alongside the client tests instead of\nin front of them, the Rust client clippy and tests share one job, the JS\nclient is tested on NODE_VERSION only, and NODE_VERSION moves to 24.x.\nThe actions side fixes the cargo caches and caches the SBF test tools.\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: repin shared workflows to the review fixes on actions ci/faster-ci\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to metaplex-foundation/actions v2.1.0\n\nCo-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>\n\n* ci: pin shared workflows to metaplex-foundation/actions v2.1.1\n\nv2.1.1 fixes the publish workflows: the shared actions checkout is no\nlonger deleted before the job's post steps run, and the publish and\nrelease steps wait for npm to serve a freshly published version.\n\nCo-Authored-By: Claude Fable 5.2 <noreply@anthropic.com>\n\n---------\n\nCo-authored-by: Claude Fable 5.1 <noreply@anthropic.com>",
          "timestamp": "2026-11-10T15:53:48-05:00",
          "tree_id": "83dd5ebab866831e5e93ec15748ed9121077fba7",
          "url": "https://github.com/metaplex-foundation/solana-project-template/commit/50fcbe04840003d7dfc113218ffa8c3fb258ba0d"
        },
        "date": 1794344235514,
        "tool": "customSmallerIsBetter",
        "benches": [
          {
            "name": "CU: create a new account",
            "value": 3734,
            "unit": "Compute Units"
          },
          {
            "name": "Space: create a new account",
            "value": 39,
            "unit": "Bytes"
          }
        ]
      }
    ]
  }
}