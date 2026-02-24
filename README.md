# reddit2xml

Token-efficient Reddit CLI tool that fetches content anonymously and outputs compact XML. Designed for use with LLM tools where every token counts.

Built to match the functionality of [mcp-reddit-anon](https://github.com/rohanp2051/mcp-reddit-anon) as a standalone CLI tool.

## Installation

### Imperative

```sh
# Install to your profile
nix profile install github:rohanp2051/reddit2xml

# Or run directly without installing
nix run github:rohanp2051/reddit2xml -- hot rust -n 5
```

### Declarative (pinned in your system or home-manager flake)

Add reddit2xml as a flake input:

```nix
# flake.nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    reddit2xml.url = "github:rohanp2051/reddit2xml";
  };
}
```

Then include the package:

```nix
# NixOS (configuration.nix)
environment.systemPackages = [ inputs.reddit2xml.packages.${system}.default ];

# Or home-manager (home.nix)
home.packages = [ inputs.reddit2xml.packages.${system}.default ];
```

### Cargo

```sh
cargo install --path .
```

## Usage

### Global options

- `-o, --output <FILE>` — Write output to a file instead of stdout

### Fetch hot posts

```sh
reddit2xml hot <SUBREDDIT> [OPTIONS]
```

Accepts a subreddit name or full URL (e.g. `rust` or `https://reddit.com/r/rust`).

**Options:**
- `-n, --limit <N>` — Number of posts (default: 10, max: 100)
- `--no-score` — Omit score
- `--no-author` — Omit author
- `--no-comments-count` — Omit comment count
- `--no-type` — Omit post type
- `--no-link` — Omit permalink
- `--no-content` — Omit content/URL
- `--no-id` — Omit post ID
- `--minimal` — Shorthand for `--no-score --no-author --no-type --no-link`
- `--titles-only` — Only titles + IDs

**Examples:**

```sh
# Get 5 hot posts from r/rust
reddit2xml hot rust -n 5

# Minimal output
reddit2xml hot rust -n 3 --minimal

# Just titles and IDs
reddit2xml hot rust --titles-only
```

### Fetch a post with comments

```sh
reddit2xml post <POST_ID_OR_URL> [OPTIONS]
```

Accepts a post ID or full URL. Comment URLs are auto-detected — the comment ID and `?context=N` are extracted automatically.

**Options:**
- `--comment <ID>` — Focus on a specific comment thread (auto-detected from URLs)
- `--context <N>` — Parent comments to include above a focused comment (auto-detected from `?context=N`)
- `-c, --comment-limit <N>` — Top-level comments (default: 20, max: 100)
- `-d, --comment-depth <N>` — Reply depth (default: 3, max: 10)
- `--no-score` — Omit scores
- `--no-author` — Omit authors
- `--no-type` — Omit post type
- `--no-comments` — Omit comments entirely
- `--minimal` — Shorthand for `--no-score --no-author --no-type`

**Examples:**

```sh
# Fetch post with 5 top-level comments, 2 levels deep
reddit2xml post 1abc2de -c 5 -d 2

# Post content only, no comments
reddit2xml post 1abc2de --no-comments

# Fetch a specific comment thread from a URL
reddit2xml post 'https://www.reddit.com/r/NixOS/comments/1ptxbny/comment/nvk950t/'

# Save output to a file
reddit2xml -o post.xml post 1abc2de
```

## XML Output

Uses short tag names to minimize tokens:

| Tag | Meaning |
|-----|---------|
| `<r>` | Response |
| `<p>` | Post |
| `<t>` | Title |
| `<c>` | Content |
| `<l>` | Link |
| `<cm>` | Comment |

| Attribute | Meaning |
|-----------|---------|
| `s` | Score |
| `nc` | Number of comments |
| `by` | Author |
