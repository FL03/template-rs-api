# template-cli-rs

[![Clippy](https://github.com/FL03/conduit/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/conduit/actions/workflows/clippy.yml)
[![Docker](https://github.com/FL03/conduit/actions/workflows/docker.yml/badge.svg)](https://github.com/FL03/conduit/actions/workflows/docker.yml)
[![Rust](https://github.com/FL03/conduit/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/conduit/actions/workflows/rust.yml)

***

conduit

## Getting Started

### Dockerfile

Make sure you have docker installed on the target system

#### *Pull the image*

```bash
docker pull jo3mccain/conduit:latest
```

#### *Build the image locally (optional)*

```bash
docker buildx build --tag conduit:latest .
```

#### *Run the image*

```bash
docker run -p 8080:8080 jo3mccain/conduit:latest
```

## Usage

```bash
    conduit  
```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
