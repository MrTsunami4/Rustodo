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
- `PUT http://localhost:3000/id/complete` pour marquer un todo comme complete
- `DELETE http://localhost:3000/id` pour supprimer un todo particulier
- `GET http://localhost:3000/protected` pour acceder a la page protetee par JWT
- `POST http://localhost:3000/register` pour creer un compte avec un body
  ```json
  {
    "username": "<username>",
    "password": "<password>"
  }
  ```
- `POST http://localhost:3000/authorize` pour recevoir un JWT avec un body
  ```json
  {
    "username": "<username>",
    "password": "<password>"
  }
  ```
