# RUST RESTful ACTIX WEB MONGODB EXAMPLE / BOILERPLATE

>__Warning__
> This example is quite old and the package versions of the libraries are outdated.
> I packaged this in a Docker, so it will still work, but please consider using a more timely example.

A small rust web server example/boilerplate with RESTful CRUD functionality, utilizing actix web and mongdb.


Inspired by [mehmetsefabalik/rust-mongodb-example](https://github.com/mehmetsefabalik/rust-mongodb-example) and [nintha/demo-myblog](https://github.com/nintha/demo-myblog).

## Prerequisites
- rust and cargo installed if using local deployment
- docker installed if using docker deployment

## Functionality
Exposes CRUD functionality for `resource`:
- `GET /resource` retrieve all `resource` objects
- `POST /resource` save a `resource` on the database
- `GET /resource/{id}` retrieve a `resource` by id
- `PUT /resource/{id}` update a `resource` by id
- `DELETE /resource/{id}` delete a `resource` by id

`POST` and `PUT` requests require the body to look like the following:
```json5
{
  "some_key_1": "a",
  "some_key_2": "b",
  "some_key_3": "c"
}
```
## Usage

### Local
1. start a `mongod` deamon locally (`mongodb://localhost:27017`)
    1. set environment variables `MONGO_HOST` and `MONGO_PORT` the appropriate values *OR* change the dynamic `mongo_connection_string` in `db.rs` to a non dynamic value
    2. set environment variable `PORT` to appropriate value *OR* change the dynamic `binding_address` in `main.rs` to a non dynamic value
2. run `cargo build --release`
3. run `./target/release/actix-mongodb-boilerplate`

### Docker
1. run `docker-compose up --build`

# CONTRIBUTE

Feel free to contribute, you may want to add extra endpoints, benchmarks, performance or quality of life enhancements, etc.
