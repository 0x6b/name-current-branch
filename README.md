# Name Current Branch

A CLI tool that automatically generates meaningful git branch names using [Claude Code](https://docs.anthropic.com/en/docs/claude-code/). Or, overengineered version of:

```bash
git branch -m "$(claude -p "Look at the staged git changes and create a descriptive git branch name, with slash for category. Only respond with the title and no affirmation.")"
```

## Usage

```console
$ name-current-branch
old-branch-name → new-branch-name
```

## How It Works

1. Get the diff between the current and the default branch
2. Use Claude Code to generate a proper-ish branch name
3. Rename the current branch

Only rename feature branches i.e. not `main`, `master`, or `develop`.

## Configuration

The tool uses an embedded TOML configuration. To customize prompts, modify [`assets/config.toml`](assets/config.toml) and rebuild:

## License

MIT. See [LICENSE](LICENSE) for details.
