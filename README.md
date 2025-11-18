# Miau Utils 🐾: A Rust Coreutils Rewrite 🦀

This project, named Miau Utils, is an ambitious, from-scratch rewrite of the essential Unix core utilities (like ls, cat, wc, cp, etc.) in the Rust programming language.

## Inspiration and Goal

This project is inspired by the excellent work of the [uutils project](https://github.com/uutils/) and the growing trend toward using Rust for critical infrastructure, such as Ubuntu's adoption of Rust-based coreutils.

I also want to gain a deep, practical understanding of advanced Rust concepts, including traits, I/O handling, error management, and performance optimization.

## Getting Started

### Prerequisites

- Rust

OR

- NixOS/Nix (optionally with `direnv`): If you use Nix, a `flake.nix` is provided to instantly set up the development environment.

### Building

1. Clone this repo:

```
git clone https://github.com/gamersekofy/miauutils.git
cd miauutils
```

2. Environment Setup (NixOS Users):

```
nix develop
# OR
direnv allow
```

3. Build the entire Miau Utils workspace:

```
cargo build --release
```

The compiled binaries will be located in `target/release/`.

## Progress Tracker

This section tracks the implementation status of each utility in the Miau Utils suite, categorized by function.

### Shell Utilities

| Command | Description | Status | Notes |
| :--- | :--- | :--- | :--- |
| `uname` | Reports system information | In-progress | Linux/macOS implementation finished |
| `arch` | Reports machine hardware name; same as `uname -m` | Planned | -- |
| `basename` | Removes the path prefix from a given pathname | Planned | -- |
| `chroot` | Changes the root directory | Planned | -- |
| `date` | Reports or sets the system date and time | Planned | -- |
| `dirname` | Strips non-directory suffix from file name | Planned | -- |
| `du` | Shows disk usage on file systems | Planned | -- |
| `echo` | Outputs text | Planned | -- |
| `env` | Reports and modifies environment variables | Planned | -- |
| `expr` | Evaluates expressions | Planned | -- |
| `factor` | Factors numbers | Planned | -- |
| `false` | Does nothing but exit with unsuccessful status | Planned | -- |
| `groups` | Reports the groups of which the user is a member | Planned | -- |
| `hostid` | Reports the numeric identifier for the current host | Planned | -- |
| `id` | Reports the real or effective UID and GID | Planned | -- |
| `link` | Creates a link to a file | Planned | -- |
| `logname` | Reports the user's login name | Planned | -- |
| `nice` | Modifies scheduling priority | Planned | -- |
| `nohup` | Allows a command to continue running after logging out | Planned | -- |
| `nproc` | Queries the number of (active) processors | Planned | -- |
| `pathchk` | Checks whether file names are valid or portable | Planned | -- |
| `pinky` | A lightweight version of finger | Planned | -- |
| `printenv` | Reports environment variables | Planned | -- |
| `printf` | Formats text | Planned | -- |
| `pwd` | Reports the current working directory | Planned | -- |
| `readlink` | Reports the value of a symbolic link | Planned | -- |
| `runcon` | Run command with specified security context | Planned | -- |
| `seq` | Reports a sequence of numbers | Planned | -- |
| `sleep` | Blocks (delays, waits) for a specified amount of time | Planned | -- |
| `stat` | Reports information about an inode | Planned | -- |
| `stdbuf` | Runs a command with custom standard streams configuration | Planned | -- |
| `stty` | Changes and reports terminal line settings | Planned | -- |
| `tee` | Sends output to multiple files | Planned | -- |
| `test` | Evaluates an expression | Planned | -- |
| `timeout` | Runs a command with a time limit | Planned | -- |
| `true` | Does nothing but exit with success status | Planned | -- |
| `tty` | Reports the terminal name | Planned | -- |
| `unlink` | Removes files via `unlink()` function | Planned | -- |
| `uptime` | Reports how long the system has been running | Planned | -- |
| `users` | Reports the user names of users currently logged into the current host | Planned | -- |
| `who` | Reports logged-in users | Planned | -- |
| `whoami` | Reports the effective userid | Planned | -- |
| `yes` | Outputs a string repeatedly | Planned | -- |
| `[` | Synonym for `test` that enables expressions like `[ expression ]` | Planned | -- |

### Text Utilities

| Command | Description | Status | Notes |
| :--- | :--- | :--- | :--- |
| `b2sum` | Computes and checks BLAKE2b message digest | Planned | -- |
| `base32` | Encodes or decodes base32 | Planned | -- |
| `base64` | Encodes or decodes base64 | Planned | -- |
| `basenc` | Encodes or decodes various encodings including hexadecimal, base32, base64, and Z85 | Planned | -- |
| `cat` | Concatenates files | Planned | -- |
| `cksum` | Report or compute the checksum of files | Planned | -- |
| `comm` | Compares two sorted files line by line | Planned | -- |
| `csplit` | Splits a file into sections determined by context lines | Planned | -- |
| `cut` | Removes sections from each line of files | Planned | -- |
| `expand` | Converts tabs to spaces | Planned | -- |
| `fmt` | Formats text | Planned | -- |
| `fold` | Wraps each input line to fit in specified width | Planned | -- |
| `head` | Outputs the first part of files | Planned | -- |
| `join` | Joins lines of two files on a common field | Planned | -- |
| `md5sum` | Computes and checks MD5 message digest | Planned | -- |
| `nl` | Numbers lines of files | Planned | -- |
| `numfmt` | Formats numbers | Planned | -- |
| `od` | Dumps files in octal and other formats | Planned | -- |
| `paste` | Merges lines of files | Planned | -- |
| `ptx` | Produces a permuted index of file contents | Planned | -- |
| `pr` | Paginates or columnates files | Planned | -- |
| `sha1sum`, `sha224sum`, `sha256sum`, `sha384sum`, `sha512sum` | Computes and checks SHA-1/SHA-2 message digests | Planned | -- |
| `shuf` | Generates random permutations | Planned | -- |
| `sort` | Sorts lines of text files | Planned | -- |
| `split` | Splits a file into pieces | Planned | -- |
| `sum` | Checksums and counts the blocks in a file | Planned | -- |
| `tac` | Concatenates files in reverse order; line by line | Planned | -- |
| `tail` | Outputs the last part of files | Planned | -- |
| `tr` | Translates or deletes characters | Planned | -- |
| `tsort` | Performs a topological sort | Planned | -- |
| `unexpand` | Converts spaces to tabs | Planned | -- |
| `uniq` | Removes duplicate lines from a sorted file | Planned | -- |
| `wc` | Reports the number of bytes, words, and lines in files | Planned | -- |

### File Utilities

| Command | Description | Status | Notes |
| :--- | :--- | :--- | :--- |
| `chcon` | Changes file security context (SELinux) | Planned | -- |
| `chgrp` | Changes file group ownership | Planned | -- |
| `chown` | Changes file user ownership | Planned | -- |
| `chmod` | Changes file permissions | Planned | -- |
| `cp` | Copies files | Planned | -- |
| `dd` | Copies and converts file data | Planned | -- |
| `df` | Reports file system free space | Planned | -- |
| `dir` | Like `ls -C -b`; by default lists files in columns, sorted vertically | Planned | -- |
| `dircolors` | Configures colors used for `ls` output | Planned | -- |
| `install` | Copies files and sets file attributes | Planned | -- |
| `ln` | Creates a link to a file | Planned | -- |
| `ls` | Lists files | Planned | -- |
| `mkdir` | Creates directories | Planned | -- |
| `mkfifo` | Creates named pipes (FIFOs) | Planned | -- |
| `mknod` | Creates block or character special files | Planned | -- |
| `mktemp` | Creates temporary regular files or directories | Planned | -- |
| `mv` | Moves and renames files | Planned | -- |
| `realpath` | Reports the absolute or relative path of a file | Planned | -- |
| `rm` | Deletes files | Planned | -- |
| `rmdir` | Deletes empty directories | Planned | -- |
| `shred` | Overwrites a file to hide its contents and optionally deletes it | Planned | -- |
| `sync` | Flushes file system buffers | Planned | -- |
| `touch` | Changes file timestamps; creating files if they do not exist | Planned | -- |
| `truncate` | Sets the size of a file via truncation or extension | Planned | -- |
| `vdir` | Like `ls -l -b`; by default lists files in long format | Planned | -- |
