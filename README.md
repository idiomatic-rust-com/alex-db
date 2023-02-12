# AlexDB

AlexDB is a basic key-value storage database that can function both as an in-memory database and as a database that writes and restores data to the filesystem.

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

## Warning

This software is in its early stages of development and future versions will feature changes in the disk data format.

## I am actively seeking employment opportunities

If you seek a Rust developer with expertise in building scalable and reliable web systems, who can deliver robust, thoroughly tested, and easily maintainable code, you can reach me at either michal@idiomatic-rust.com or idiomatic.rust.com@gmail.com.
