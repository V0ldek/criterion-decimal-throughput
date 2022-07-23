# How to contribute

- Found a bug? Check if it was reported yet as a
[![GitHub issues by-label](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/bug?color=%23d73a4a&label=bug&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/bug),
if not then file an [issue](https://github.com/V0ldek/criterion-decimal-throughput/issues/new)
- Want a feature? Check if it's already proposed as an
[![enhancement issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/enhancement?color=%23a2eeef&label=enhancement&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/enhancement)
, if not then file an [issue](https://github.com/V0ldek/criterion-decimal-throughput/issues/new)

Every newly created issue gets assigned the
[![triage issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/triage?color=%2384A6B5&label=triage&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/triage)
label. Once reviewed,
I exchange it for
[![go ahead issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/go%20ahead?color=%23FF4400&label=go%20ahead&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/go%20ahead)
to signal it is of value to the project and
can be worked on (for a feature), or that it is indeed a bug that we need to fix (for a bug).

## Code contributions

You want to write code for the crate? Great! First, you need an issue to contribute to,
one marked as
[![go ahead issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/go%20ahead?color=%23FF4400&label=go%20ahead&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/go%20ahead).
You can also use
[![help wanted issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/help%20wanted?color=%23008672&label=help%20wanted&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/help%20wanted),
meaning "I'd be very happy if someone implemented this",
or
[![good first issue issues](https://img.shields.io/github/issues/v0ldek/criterion-decimal-throughput/good%20first%20issue?color=%237057ff&label=good%20first%20issue&logo=github)](https://github.com/v0ldek/criterion-decimal-throughput/labels/good%20first%20issue),
meaning "I'd be happy if someone implemented this and it's relatively straightforward".
Go to the issue and post a comment that you're going to work on it. [Fork the repo](https://github.com/V0ldek/criterion-decimal-throughput/fork),
write your feature of fix, then create a PR.

### Setting up local development

Nothing special, you just need `cargo`.

### Guidelines

1. Use standard `rustfmt` settings for formatting.
2. Lint your code with [`clippy`](https://github.com/rust-lang/rust-clippy).
3. Follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/), more on it below.
4. Avoid adding new dependencies to Cargo.toml unless there is a good reason for them.

### Commit messages

Your commit messages should look like this:

```ini
type: short description of changes

More detailed description of my change.
Potentially multiline.

Refs: #69

# This project follows Conventional Commits 1.0.0 (https://www.conventionalcommits.org/en/v1.0.0/)
#
# The first line should be a short description of the purpose of the commit.
# Allowed types are: build, ci, docs, feat, fix, perf, refactor, style, test, chore
#
###
# Note: If introducing a breaking change, add an exclamation mark after the type
# Example: fix!: removed `AsRef` impl in favour of `relax_alignment`
### 
# The second section contains a detailed description.
# Always give a description for features,
# omit only for real one-liners like dependency bumps.
# Footer are at the end. Most important are refs, 
# which tell us which GitHub issue the change is related to, if any.
```
