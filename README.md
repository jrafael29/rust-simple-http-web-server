## Rust Single-Threaded Web Server

A simple HTTP web server built from scratch in Rust, without using a web framework.

The project is a learning exercise focused on understanding the basics of HTTP request handling, TCP connections and JSON serialization.

### Features

* HTML route: Serves an HTML page.
* JSON route: Returns user data as JSON.
* User creation ednpoint: Stores user data temporarily in memory.
* Manual HTTP handling: Parses HTTP requests and builds HTTP responses without a web framework.

### Tecnologies:

* Rust
* Serde JSON
* Serde

## Start

### Prerequisites
* Rust and Cargo

### Running

```bash
cargo run
```

### Limitations

* Single-threaded request handling
* User data is stored in memory and is lost when the server stops
* HTTP request parsing and response generation are implemented manually
* This project is intended for learning purposes.

## Testing

The serve exposes the following routes:

### Create a user

POST endpoint `/user` is available to receive `username` and `age` fields. 
```bash
curl -H "Content-Type: application/json" -d '{"username":"josee","age":20}' http://127.0.0.1:3005/user
```

### List users

GET route `/user` is available to list all users
```bash
curl http://localhost:3005/
```

### Access the HTML Page

GET route `/` is available to form to store a user

Open the following URL in your browser:

http://127.0.0.1:3005/

## Referencies: 

[The Rust Programming Language — Building a Single-Threaded Web Server](https://doc.rust-lang.org/book/ch21-01-single-threaded.html)

[Translating a JSON object into a HashMap with Serde JSON](https://stackoverflow.com/questions/70342603/translating-json-object-into-hashmap-with-serde-json)


__*By José Rafael © 2026*__