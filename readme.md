# MyAxumServ

Concerns are separated in this way, using the "List Users" (`GET /api/v1/users`) operation as an example:

![](docs/images/sequence-diagram-list-users.png)

(For simplicity, I've implemented a simple file data store, to serve as a plain, local DB client)

## Operations

### GET:list_users

```shell
curl localhost:3000/api/v1/users
```

### POST:create_user

```shell
curl localhost:3000/api/v1/users -H "Content-Type: application/json" --data "{\"username\":\"myUsername\"}"
```

## Dependencies

### Web framework
- **axum**: The star of the show! This web service is built using this framework.
- **tokio**: Transitive dependency of axum. It's only main use here is the annotation on the main() method.

### ORM
- **serde**: Axum uses this to serialize and deserialize structs. In this application, it's mostly used to label things.
- **serde_json**: Only used for reading and writing from the `resources/local_db.json` file, for DB storage.
  
## Utility
- **uuid**: To set user ID fields as UUIDv4 strings. 