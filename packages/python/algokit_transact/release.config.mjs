export default {
  branches: ["main", { name: "alpha", prerelease: true }],
  repositoryUrl: "https://github.com/joe-p/algokit-core",
  tagFormat: "python/algokit_transact@${version}",
  plugins: [
    [
      "semantic-release-scope-filter",
      {
        scopes: [
          "python",
          "python/algokit_transact",
          "algokit_transact",
          "algokit_transact_ffi",
        ],
        filterOutMissingScope: false,
      },
    ],
    [
      "@semantic-release/commit-analyzer",
      {
        preset: "angular",
      },
    ],
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/github",
      {
        assets: ["../../../artifacts/*-wheel/**/*"],
      },
    ],
    "semantic-release-gha-output",
  ],
};
