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

| Variable | Description | Required |
|---|---|---|
| `GITLAB_API_URL` | GitLab API base URL (e.g. `https://gitlab.com/api/v4`) | Yes |
| `GITLAB_PRIVATE_TOKEN` | Personal access token | One of the two |
| `CI_JOB_TOKEN` | CI job token (for use in pipelines) | |

### CLI Options

CLI options take priority over environment variables.

| Flag | Description | Env Fallback |
|---|---|---|
| `--api-url` | GitLab API base URL | `GITLAB_API_URL` |
| `--token` | Access token (uses `PRIVATE-TOKEN` header) | `GITLAB_PRIVATE_TOKEN` |
| `--insecure` | Skip TLS certificate verification | - |

## Usage

### Quick Start

```bash
# 1. Set environment variables
export GITLAB_API_URL=https://gitlab.example.com/api/v4
export GITLAB_PRIVATE_TOKEN=glpat-xxxxxxxxxxxx

# 2. Run commands directly
gitlab-cli artifacts download --project my-group/my-project --branch main --job build
```

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

# Override API config via CLI
gitlab-cli \
  --api-url https://gitlab.example.com/api/v4 \
  --token glpat-xxxxxxxxxxxx \
  --insecure \
  artifacts download \
  --project my-group/my-project \
  --branch main \
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
