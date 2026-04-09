# gitlab-cli

A CLI tool for interacting with GitLab API, written in Rust.

## Features

- **Artifacts** — Download CI/CD pipeline artifacts by branch or commit
- **Package Registry** — Upload, download, list, and delete generic packages

## Installation

```bash
cargo build --release
cp target/release/gitlab-cli /usr/local/bin/
```

## Configuration

Set the following environment variables:

| Variable | Description | Required |
|---|---|---|
| `GITLAB_API_URL` | GitLab API base URL (e.g. `https://gitlab.com/api/v4`) | Yes |
| `GITLAB_PRIVATE_TOKEN` | Personal access token | One of the three |
| `GITLAB_TOKEN` | Access token (alias) | |
| `CI_JOB_TOKEN` | CI job token (for use in pipelines) | |

## Usage

### Artifacts

Download artifacts from a specific job in the latest pipeline of a branch or commit.

```bash
# By branch
gitlab-cli artifacts download \
  --project my-group/my-project \
  --branch main \
  --job build \
  --output artifacts.zip

# By commit
gitlab-cli artifacts download \
  --project my-group/my-project \
  --commit abc123def456 \
  --job build \
  --output artifacts.zip
```

### Package Registry

#### Upload

```bash
gitlab-cli package upload \
  --project my-group/my-project \
  --name my-package \
  --version 1.0.0 \
  --file ./dist/binary.tar.gz
```

#### Download

```bash
gitlab-cli package download \
  --project my-group/my-project \
  --name my-package \
  --version 1.0.0 \
  --file binary.tar.gz \
  --output ./binary.tar.gz
```

#### List

```bash
# List all packages
gitlab-cli package list --project my-group/my-project

# Filter by name
gitlab-cli package list --project my-group/my-project --name my-package
```

#### Delete

```bash
gitlab-cli package delete \
  --project my-group/my-project \
  --id 42
```

### Global Options

| Flag | Description |
|---|---|
| `--insecure` | Skip TLS certificate verification (for self-signed certificates) |
