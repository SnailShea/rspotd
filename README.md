# RsPOTD

Rust library to generate an ARRIS-compatible password of the day.
Inspired by [arrispwgen](https://github.com/borfast/arrispwgen).

The original author essentially reverse-engineered the algorithm after poking modems. Thank you so much @borfast for this incredible work. I ran into a few niche issues based on my needs/specific seed that I hope this project will rectify.

First, the upstream project seems to lock you to a seed between 8-10 characters. The original ARRIS tooling to generate these passwords actually only allows seeds between 4-8 characters. (I find this strange, considering that the default ARRIS seed is 10 characters in length)

Second, if you supply a seed of less than 10 characters, the upstream project will present a password of the same number of characters. So an 8 character seed will yield the _correct_ first 8 characters of the password, but will not provide the _full_ password.

Third, if you supply a seed of less than 8 characters, the upstream project does not appear to output anything at all. The way ARRIS handles the seeds is to iterate through the seed, appending each character to the end of the supplied seed. For example, a seed of "ABCD" is a _valid seed_, however in effect it will become "ABCDABCDAB".

Fourth, as much as I would love to contribute upstream fixes rather than a fork/port, the upstream project was written in TypeScript and packaged via NPM. I do not have experience with either of these technologies, and as such a port became much more viable.

# Usage

## Single date

The `generate()` function accepts two keyword arguments.

- `date`: Accepts a single date in ISO format (i.e., 2021-07-23)
- `seed`: Accepts a seed between 4 and 8 characters

### Using ARRIS default seed

```rust
use rspotd::{DEFAULT_SEED, generate};

generate("2021-12-25", DEFAULT_SEED);
```

Output (Assuming date is 2021-07-23): `O9W2Q1O16V`

### Using custom seed

```rust
use rspotd::generate;

generate("2021-07-23", "ABCDABCD");
```

Output: `F32CAZCJLU`

## Range of dates

The `generate_multiple()` function takes a start and end date, as well as an optional seed.

### Using default seed

```rust
use rspotd::{DEFAULT_SEED, generate_multiple};

generate_multiple("2021-07-23", "2022-07-28", DEFAULT_SEED);
```

Output (truncated):

```
{
    '07/23/21': 'O9W2Q1O16V',
    '07/24/21': '2SEIWWLZL1',
    '07/25/21': 'ZOU3MWRZN0',
    ...
}
```

### Using custom seed

```rust
use rspotd::generate_multiple;

generate_multiple("2021-07-23", "2022-07-28", "ABCDABCD");
```

Output (truncated):

```
{
    '07/23/21': 'F32CAZCJLU',
    '07/24/21': '95L0GFUNCC',
    '07/25/21': 'CSBLM119CH',
    ...
}
```

## Support for wasm-bindgen

Enable the `wasm` feature when adding rspotd to your project

```toml
[dependencies]
rspotd = { version = "0.1.13", features = ["wasm"] }

```
