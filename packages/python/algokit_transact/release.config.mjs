import { setOutput } from "@actions/core";

export default {
  branches: ["main", { name: "alpha", prerelease: true }],
  repositoryUrl: "https://github.com/joe-p/algokit-core",
  tagFormat: "python/algokit_transact@${version}",
  plugins: [
    [
      "semantic-release-scope-filter",
      {
        scopes: ["python", "python/algokit_transact", "algokit_transact"],
        filterOutMissingScope: false,
      },
    ],
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "angular",
      },
    ],
    "semantic-release-gha-output",
    "@semantic-release/release-notes-generator",
    {
      generateNotes: async (_cfg, context) => {
        setOutput("notes", context.nextRelease.notes);
      },
    },
  ],
};
