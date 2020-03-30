# RUST RESTful ACTIX WEB MONGODB EXAMPLE / BOILERPLATE

A minimal rust web server example/boilerplate with RESTful CRUD functionality, utilizing actix web and mongdb.

Inspired by [mehmetsefabalik/rust-mongodb-example](https://github.com/mehmetsefabalik/rust-mongodb-example).

## Prerequisites
- ssl installed (e. g. OpenSSL) if using local deployment
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
2. run `cargo build --release`
3. run `./target/release/actix-mongodb-boilerplate`

### Docker
1. run `docker-compose up --build`

### API commands

`curl http://localhost:3000/hello?name=joe`

`Success` should be returned and `{name: "joe"}` document should have been written in `mydb` database, `users` collection.

# CONTRIBUTE

Feel free to contribute, you may want to add extra endpoints, benchmark against different stacks, etc.