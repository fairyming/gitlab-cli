# gitlab-cli

A CLI tool for interacting with GitLab API, written in Rust.

## Features

- **Artifacts** — Download CI/CD pipeline artifacts by branch or commit
- **Pipeline** — Trigger a specific job in a pipeline by branch or commit, with custom environment variables and job inputs
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
| `--output` | Output file path for the zip archive, or output directory when `--extract` is used | `artifacts.zip` / `.` (with `--extract`) |
| `--extract` | Extract the zip archive after downloading; `--output` becomes the destination directory | `false` |

```bash
# Download by branch
gitlab-cli artifacts download \
  --project my-group/my-project \
  --branch main \
  --job build \
  --output artifacts.zip

# Download by branch and extract to current directory
gitlab-cli artifacts download \
  --project my-group/my-project \
  --branch main \
  --job build \
  --extract

# Download by branch and extract to a specific directory
gitlab-cli artifacts download \
  --project my-group/my-project \
  --branch main \
  --job build \
  --output ./dist \
  --extract

# Download by commit
gitlab-cli artifacts download \
  --project my-group/my-project \
  --commit abc123def456 \
  --job build
```

### Pipeline

Trigger a specific job in a pipeline by branch or commit. Supports passing custom environment variables and job inputs.

**Required arguments:**

| Argument | Description |
|---|---|
| `--project` | Project ID or path (e.g. `my-group/my-project`) |
| `--job` | Job name to trigger |

**Specify pipeline by ref (choose one):**

| Argument | Description |
|---|---|
| `--branch` | Branch name to find the latest pipeline |
| `--commit` | Commit SHA to find the latest pipeline |

**Optional arguments:**

| Argument | Description |
|---|---|
| `--env KEY=VALUE` | Environment variables (can be specified multiple times) |
| `--input KEY=VALUE` | Job inputs for interactive/manual jobs (can be specified multiple times) |

The tool will find the job in the pipeline and:
- **Manual job** → play the job
- **Failed/canceled/skipped/success job** → retry the job
- **Running/pending job** → exit with error

```bash
# Trigger a job by branch with variables
gitlab-cli --use-private-token pipeline run \
  --project my-group/my-project \
  --branch main \
  --job deploy \
  --env FOO=bar \
  --env BAZ=qux

# Trigger a job with job inputs
gitlab-cli --use-private-token pipeline run \
  --project my-group/my-project \
  --branch main \
  --job deploy \
  --input environment=staging \
  --input version=v2.1.0

# Trigger a job by commit with both variables and inputs
gitlab-cli --use-private-token pipeline run \
  --project my-group/my-project \
  --commit abc123def456 \
  --job build \
  --input filter=xml
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
