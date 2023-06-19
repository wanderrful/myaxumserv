# MyAxumServ

Concerns are separated in this way, using the "List Users" (`GET /api/v1/users`) operation as an example:

![](docs/images/sequence-diagram-list-users.png)

(For simplicity, I've implemented a simple file data store, to serve as a plain, local DB client)

## Operations

### GET

```shell
curl localhost:3000/api/v1/users
```

### POST

```shell
curl localhost:3000/api/v1/users -H "Content-Type: application/json" --data "{\"username\":\"myUsername\"}"
```
