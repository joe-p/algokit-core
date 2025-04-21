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
  ],
};
