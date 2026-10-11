use std::collections::HashMap;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::sync::{Arc, Mutex};
use serde::{Serialize};
use serde_json::{Result, Value};

#[derive(Serialize, Debug, Clone)]
struct User {
    username: String,
    age: u64
}

// #[derive(Serialize, Debug, Clone)]
// struct Request {
//     headers: HashMap<String, String>,
//     method: String,
//     path: String,
//     body: String
// }

type InMemoryDb = Arc<Mutex<Vec<User>>>;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3005");
    let users_vec: InMemoryDb = Arc::new(Mutex::new(Vec::<User>::new()));
    match &listener {
        Ok(_) => {
            for stream in listener.expect("Error listener").incoming() {
                let stream: TcpStream = stream.unwrap();
                let users = Arc::clone(&users_vec);
                println!("conexao aberta");
                handle_request(stream, users);
            }
        },
        Err(e) => {
            println!("Deu ruim: {}", e);
        }
    }
}

fn handle_request(stream: TcpStream, users: InMemoryDb) -> () {
    let mut buf_reader: BufReader<&TcpStream> = BufReader::new(&stream);

    let mut request_line = String::new();
    buf_reader.read_line(&mut request_line).unwrap();
    println!("request_line: {}", request_line);

    let mut request_parts = request_line.split_whitespace();
    let method = request_parts.next().expect("erro ao obter metodo");
    let route = request_parts.next().expect("erro ao obter rota");
    let version = request_parts.next().expect("erro ao obter version");

    println!("Method: {} | Route: {} | version: {}", method, route, version);

    if method.is_empty() || route.is_empty() || version.is_empty() {
        return;
    }


    // headers
    let headers: HashMap<String, String> = transform_headers_to_hashmap(&mut buf_reader);

    let content_length: usize = headers
        .get("content-length")
        .and_then(|value| value.parse().ok())
        .unwrap_or(0);

    println!("Headers hashmap: {headers:#?}");
    

    // body
    let mut body = vec![0; content_length];
    buf_reader.read_exact(&mut body).unwrap();

    let body = String::from_utf8(body).unwrap();

    // println!("Corpo da requisição: {body}");

    let body_hashmap: HashMap<String, Value>;
    if !body.is_empty() {
        body_hashmap = json_to_hashmap(&body.as_str(), vec!["username", "age"]).unwrap();
    }else{
        body_hashmap = HashMap::new()
    }

    println!("Body hashmap: {body_hashmap:?}");
    // for (key, value) in headers {
    //     println!("Key: {}, Value: {}", key, value);
    // }

    // let request = Request {
    //     method: String::from(method),
    //     path: String::from(route),
    //     headers: headers,
    //     body: body
    // };

    
    match (method, route) {
        ("GET", "/") => {
            println!("acessando root");
            return response_view(stream, "index.html");
        },
        ("GET", "/user") => {
            let users_d = users.lock().unwrap();
            let user_body = serde_json::to_string(&users_d[..]).unwrap();

            return response_json(stream, user_body.as_str())
        },
        ("POST", "/user") => {

            if body_hashmap.get("name").is_none() || body_hashmap.get("age").is_none() {
                println!("dados invalidos para criação.");
                let body= serde_json::json!({"error": "fields required"}).to_string();
                return response_json(
                    stream, 
                    &body
                );
            } 

            let mut usrs = users.lock().unwrap();
            let usr = User {
                username: body_hashmap.get("username").unwrap().to_string(),
                age: body_hashmap.get("age").unwrap().as_u64().unwrap()
            };
            usrs.push(usr);
            println!("inseriu user");
        }
        _ => {
            return response_view(stream, "404.html");
        }
    }

}
fn response_view(mut stream: TcpStream, view: &str) -> () {

    let mut view_path = String::new();
    view_path.push_str("src/views/");
    view_path.push_str(view);

    let status_line: &str = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string(view_path).expect("erro ao carregar arquivo index.html");
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn response_json(mut stream: TcpStream, body: &str) -> () {

    let status_line: &str = "HTTP/1.1 200 OK";
    let length = body.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn transform_headers_to_hashmap(buf_reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
    // let mut headers: HashMap<&str, &str> = HashMap::new();
    let mut headers = HashMap::new();

    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();

        // fim dos headers
        if line == "\r\n" || line.is_empty() {
            break;
        }

        if let Some((key, value)) = line.split_once(":") {
            let key = key.trim().to_ascii_lowercase();
            let value = value.trim().to_string();

            headers.insert(key, value);
        }
    }

    return headers;
}
/**
 * referenced on: https://stackoverflow.com/questions/70342603/translating-json-object-into-hashmap-with-serde-json
 */
fn json_to_hashmap(json: &str, keys: Vec<&str>) -> Result<HashMap<String, Value>> {
    let mut lookup: HashMap<String, Value> = serde_json::from_str(json).expect("erro ao ler json");
    let mut map: HashMap<String, Value> = HashMap::new();
    for key in keys {
        if lookup.get(key).is_some() {
            let (k, v) = lookup.remove_entry(key).expect("erro ao ler json");
            map.insert(k, v);
        }
    }
    Ok(map)
}
