# Thoth

A custom unicode graphme clusters segmentation implementation without emoji support.

> [!important]
> Ignores rules `GB9c` through to `GB13` of the `Grapheme Cluster Boundary Rules`. 
> This is done for simplicity.
> 
> However, this also means that emoji (including country flags), along with 'not breaking within certain combinations with Indic_Conjunct_Break' of extended clusters are *not* supported.
> 
> This means that `Thoth` is not a true Unicode grapheme segmentation algorithm.
> It also means that neither extended nor legacy grapheme clusters are truly supported by `Thoth` according to the Unicode specification.

It follows my "All code written by me or part of rust's standard library and libc" philosophy.
You can learn more about that [here](https://blog.xqhare.net/posts/why_solve_problems/).

## Features

- _**No dependencies**_: All code is written by me or part of std.
- Segments text into grapheme clusters.

## Roadmap

`Thoth` uses my [nomos](https://github.com/xqhare/nomos) project management system.

The roadmap for this project can be found in the [nomos.md](nomos.md) file.

All nomos files follow the syntax defined [here](https://github.com/Xqhare/nomos/blob/master/spec/).

## Naming

As with all my projects, Thoth is named after an ancient deity.
Learn more about my naming scheme [here](https://blog.xqhare.net/posts/explaining_the_pantheon/).

Thoth, is one of the most prominent ancient Egyptian dieties of writing, magic and wisdom. He is credited with inventing hieroglyphics while serving as the scribe of the gods. He was also a divine architect and cosmic lawmaker organising sacred time and space to align with universal balance.

## Contained `bin`

- `pre_generator`: Generates the `GraphemeBreakProperty.txt` file used by the `thoth` crate. Only needed for development, not general usage.

## Usage

### Importing

Add the following to your `Cargo.toml`:

```toml
[dependencies]
thoth = { git = "https://github.com/xqhare/thoth" }
```

### Example

While `Thoth` can error, this is mainly caused by invalid Unicode characters or byte sequences.

```rust
use thoth::Thoth;

fn main() {
    let text = "Hello World!";
    assert_eq!(
        Thoth::grapheme_segmentation(text).unwrap(),
        vec!["H", "e", "l", "l", "o", " ", "W", "o", "r", "l", "d", "!"]
    );
}
```

## License

Thoth is distributed under the [MIT](https://github.com/xqhare/thoth/blob/master/LICENSE) license.

## Contributing

See [CONTRIBUTING](https://github.com/xqhare/thoth/blob/master/CONTRIBUTING.md) for contribution guidelines.
