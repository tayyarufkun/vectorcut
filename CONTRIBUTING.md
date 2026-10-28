# Contributing to VectorCut Studio

Thank you for your interest in contributing to VectorCut Studio! We welcome bug reports, feature suggestions, documentation improvements, and code contributions from everyone.

---

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## How Can I Contribute?

### Reporting Bugs

Before creating a bug report, please check existing issues to ensure the problem hasn't already been reported. When creating an issue, please include:

* A clear and descriptive title.
* Detailed steps to reproduce the behavior.
* What you expected to happen versus what actually happened.
* Operating system, version, and hardware architecture (e.g. macOS 15, Apple Silicon).
* A sample image or PDF file if relevant to the conversion error.

### Suggesting Enhancements

Feature requests are tracked as GitHub issues. Please provide:

* A clear explanation of the feature and the problem it solves.
* How the enhancement would benefit other users.
* Mockups or workflow examples if applicable.

---

## Local Development Workflow

### Prerequisites

* **Node.js** >= 18 and **npm**
* **Rust** stable toolchain (`rustup`)
* Platform requirements for Tauri (see [Tauri Guides](https://v2.tauri.app/start/prerequisites/))

### Initial Setup

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/<your-username>/vectorcut.git
   cd vectorcut
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Launch the desktop application in live development mode:
   ```bash
   npm run tauri dev
   ```

### Running Tests

Before submitting code, please ensure all unit tests pass:

```bash
# Frontend build verification
npm run build

# Rust unit tests
cd src-tauri && cargo test
```

---

## Pull Request Guidelines

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/my-new-feature
   ```
2. Keep pull requests focused on a single change.
3. Follow existing code formatting standards (`cargo fmt` for Rust, Prettier for TypeScript/Svelte).
4. Write descriptive commit messages following the [Conventional Commits](https://www.conventionalcommits.org/) specification (e.g., `feat:`, `fix:`, `docs:`, `refactor:`).
5. Open a Pull Request and fill out the provided PR template.

Thank you for helping make VectorCut Studio better!
