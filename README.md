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

### Environment Variables

| Variable | Description |
|---|---|
| `CI_API_V4_URL` | GitLab API base URL (e.g. `https://gitlab.com/api/v4`) |
| `GITLAB_PRIVATE_TOKEN` | Personal access token |
| `CI_JOB_TOKEN` | CI job token |

### CLI Options

CLI options take priority over environment variables.

| Flag | Description | Env Fallback |
|---|---|---|
| `--api-url` | GitLab API base URL | `CI_API_V4_URL` |
| `--use-private-token` | Switch to `PRIVATE-TOKEN` auth mode | - |
| `--private-token` | Personal access token (requires `--use-private-token`) | `GITLAB_PRIVATE_TOKEN` |
| `--job-token` | CI job token (default mode) | `CI_JOB_TOKEN` |
| `--insecure` | Skip TLS certificate verification | - |
| `--log-level` | HTTP tracing log level (trace/debug/info/warn/error) | `warn` |

### Auth Mode

By default, the CLI uses `CI_JOB_TOKEN` (header: `JOB-TOKEN`) for authentication.

Add `--use-private-token` to switch to `GITLAB_PRIVATE_TOKEN` (header: `PRIVATE-TOKEN`) mode.

```bash
# Default: uses CI_JOB_TOKEN (from env or --job-token)
gitlab-cli artifacts download --project my-group/my-project --branch main --job build

# Explicit: uses GITLAB_PRIVATE_TOKEN (from env or --private-token)
gitlab-cli --use-private-token artifacts download --project my-group/my-project --branch main --job build

# Override token via CLI
gitlab-cli --use-private-token --private-token glpat-xxxx \
  --api-url https://gitlab.example.com/api/v4 \
  artifacts download --project my-group/my-project --branch main --job build
```

## Usage

### Artifacts

Download artifacts from a specific job in the latest pipeline of a branch or commit. Artifacts are saved as a zip file.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path (e.g. `my-group/my-project`) |
| `--job` | Job name to download artifacts from |

**Specify pipeline by ref (choose one):**

| Argument | Description |
|---|---|
| `--branch` | Branch name to find the latest pipeline |
| `--commit` | Commit SHA to find the latest pipeline |

**Optional arguments:**

| Argument | Description | Default |
|---|---|---|
| `--output` | Output file path for the zip archive | `artifacts.zip` |

```bash
# Download by branch
gitlab-cli artifacts download \
  --project my-group/my-project \
  --branch main \
  --job build \
  --output artifacts.zip

# Download by commit
gitlab-cli artifacts download \
  --project my-group/my-project \
  --commit abc123def456 \
  --job build
```

### Package Registry

#### Upload

Upload a file as a generic package.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path |
| `--name` | Package name |
| `--version` | Package version |
| `--file` | File path to upload |

```bash
gitlab-cli package upload \
  --project my-group/my-project \
  --name my-package \
  --version 1.0.0 \
  --file ./dist/binary.tar.gz
```

#### Download

Download a specific file from a generic package.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path |
| `--name` | Package name |
| `--version` | Package version |
| `--file` | File name in the package registry |
| `--output` | Output file path on local disk |

```bash
gitlab-cli package download \
  --project my-group/my-project \
  --name my-package \
  --version 1.0.0 \
  --file binary.tar.gz \
  --output ./binary.tar.gz
```

#### List

List packages in a project.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path |

**Optional arguments:**

| Argument | Description |
|---|---|
| `--name` | Filter by package name |

```bash
# List all packages
gitlab-cli package list --project my-group/my-project

# Filter by name
gitlab-cli package list --project my-group/my-project --name my-package
```

#### Delete

Delete a package by ID.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path |
| `--id` | Package ID |

```bash
gitlab-cli package delete \
  --project my-group/my-project \
  --id 42
```
