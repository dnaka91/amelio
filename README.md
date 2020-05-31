# Amelio

<!-- markdownlint-disable no-inline-html -->
<img src="assets/images/logo@4x.png" width="215" height="56">
<!-- markdownlint-restore -->

![CI](https://github.com/dnaka91/amelio/workflows/CI/badge.svg?branch=master)

Amelio is a group project for the IUBH in Germany. It is a ticket system that helps to report and
track errors in study media.

The name Amelio is a short version of _[ameliorate](https://www.dictionary.com/browse/ameliorate)_
and is another word for **improve**.

## Build

Have the latest `rustup`, `rust` toolchain and `cargo` installed and run:

```sh
cargo build
```

## Docker

The project contains a `Dockerfile` so you can build it as an independent image:

```sh
docker build -t amelio .
```

Make sure to enable BuildKit for your Docker instance as the file contains experimental features.
Alternatively you can pull a pre-generated image: `docker pull dnaka91/amelio`.

## License

This project is licensed under [GPL License](LICENSE) (or
<https://www.gnu.org/licenses/gpl-3.0.en.html>).
