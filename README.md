# google-draco

Rust wrapper crate for the [Google Draco](https://github.com/google/draco) mesh compression library. Not really working yet.

## Usage
_TODO_

## Development
### Updating the bindings
The bindings are generated with `rust-bindgen`, which requires clang/llvm. For easier updates without complicated dependency setup, Docker and Docker Compose can be used:
```
docker-compose run generate cargo build
```
(updates `src/bindings.rs`)
