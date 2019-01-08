# mdbda

Lambda function that compiles markdown with [comrak](https://github.com/kivikakk/comrak).

### Usage

The event has a mandatory `input` field:

```json
{
  "input": "# Hello"
}
```

And another `options` field, that matches [comrak](https://docs.rs/comrak/0.4.0/comrak/struct.ComrakOptions.html) options, but camelCased.

```json
{
  "input": "# Hello",
  "options": {
    "width": 30
  }
}
```

### Build

Install the `x86_64-unknown-linux-musl` target:

```shell
rustup target add x86_64-unknown-linux-musl
```

Install [just](https://github.com/casey/just):

```shell
cargo install just
```

Compile and zip:

```shell
just
```
