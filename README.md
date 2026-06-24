# Thoth

TODO:

- Consider ArgosCI integration
- Consider needed dependencies in `Cargo.toml`

A custom unicode graphme clusters segmentation implementation.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- _**No dependencies**_: All code is written by me or part of std.

## Environment

Thoth expects the environment to provide:

- `ls` UNIX command

## Roadmap

`Thoth` uses my [nomos](https://github.com/xqhare/nomos) project management system.

The roadmap for this project can be found in the [nomos.md](nomos.md) file.

All nomos files follow the syntax defined [here](https://github.com/Xqhare/nomos/blob/master/spec/).

## Naming

As with all my projects, Thoth is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Thoth, is one of the most prominent ancient Egyptian dieties of writing, magic and wisdom. He is credited with inventing hieroglyphics while serving as the scribe of the gods. He was also a divine architect and cosmic lawmaker organising sacred time and space to align with universal balance.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
thoth = { git = "https://github.com/xqhare/thoth" }
```

### Example

```rust

```

## License

Thoth is distributed under the [MIT](https://github.com/xqhare/thoth/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/thoth/blob/master/CONTRIBUTING.md) for contribution guidelines.
