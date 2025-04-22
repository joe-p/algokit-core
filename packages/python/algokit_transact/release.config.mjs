import { readFileSync, writeFileSync } from "fs";
import { exec } from "@actions/exec";

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
    // Plugin for bumping version number
    {
      prepare: async (_pluginConfig, context) => {
        const file = "./pyproject.toml";
        const { version } = context.nextRelease;

        writeFileSync(
          file,
          readFileSync(file, "utf8").replace(
            /version =.*/,
            `version = "${version}"`,
          ),
        );

        await exec("git", ["add", file]);
        await exec("git", [
          "commit",
          "-m",
          `chore(python/algokit_transact): bump version to ${version}`,
        ]);
      },
    },
  ],
};
