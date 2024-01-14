# cgit

`cgit` is a utility that acts as a wrapper around `git clone`, allowing you to organize project folders in a reverse domain-like structure.

## Usage

Assuming you have a repository at `https://gitsite.com/username/repo-name.git`, you can use `cgit` as follows:

```bash
cgit https://gitsite.com/username/repo-name.git
```

This command is equivalent to:

```bash
git \
clone \
https://gitsite.com/username/repo-name.git \
com.gitsite.username.repo-name/repo_name
```

The format of th above commands is as follows:

```bash
git \
clone \
<repo url> \
<repo reverse domain folder>/<repo name>
```

`cgit` is also compatible with repositories in the format: `git@gitsite.com:username/repo-name.git`.

## Installation

To use `cgit`, follow these steps:

1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Globally install the `cargo-run-script` Rust crate:

    ```bash
    cargo install cargo-run-script
    ```

3. This will install the `cgit` executable globally in the `/usr/local/bin` directory:

    ```bash
    cargo run-script install
    ```

4. After installation, `cgit` should be available in a new terminal session.
