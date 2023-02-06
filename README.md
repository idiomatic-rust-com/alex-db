# AlexDB

AlexDB is a basic key-value storage database.

## Key features (v0.1)

- Basic API authentication
- Data operations:
  - append
  - create
  - decrement
  - delete
  - increment
  - list
  - pop-back
  - pop-front
  - prepend
  - read
  - update
- Data types:
  - array (can be nested, mixed values)
  - bool
  - integer
  - string
- Indexes that allows sorting:
  - created_at
  - delete_at
  - key
  - updated_at
- Pagination support
- Value expiration

## Planned features

- ACID?
- RAFT?
- Semantic search?
- WAL?

[alex-db-client](alex-db-client) - is a command-line application that facilitates communication with the database server.

[alex-db-lib](alex-db-lib) - is a compact, standalone database engine designed for embedding/integration into other systems.

[alex-db-server](alex-db-server) - is a database server application offering REST API for communication.
