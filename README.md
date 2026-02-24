# reddit2xml

Token-efficient Reddit CLI tool that fetches content anonymously and outputs compact XML. Designed for use with LLM tools where every token counts.

## Installation

### Nix flake

```sh
nix profile install github:rohanp/reddit2xml
```

Or run directly:

```sh
nix run github:rohanp/reddit2xml -- hot rust -n 5
```

### Cargo

```sh
cargo install --path .
```

## Usage

### Fetch hot posts

```sh
reddit2xml hot <SUBREDDIT> [OPTIONS]
```

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
reddit2xml post <POST_ID> [OPTIONS]
```

**Options:**
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
