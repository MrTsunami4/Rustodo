# Rustodo

## Installation
```bash
cargo run --release
```

## Routes

- `GET http://localhost:3000/` pour lire tout les todos
- `POST http://localhost:3000/` avec un body
  ```json
  {
    "content": "<Test to add>"
  }
  ```
- `GET http://localhost:3000/id` pour avoir un todo particulier
- `DELETE http://localhost:3000/id` pour supprimer un todo particulier
