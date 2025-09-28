use parsing::{parse_json, JsonType, FromJson};

use std::{
    collections::HashMap, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}
};

use parsing::{HttpRequest, HttpResponse, KnownHeader, HttpContentType, HttpPlatform, HttpStatusCode, read_http_request, write_http_response};

fn main() {

    let platform = HttpPlatform::new(|req| {
        println!("Handling request for path: {}", req.path.full_path);
        let mut headers = HashMap::<String, KnownHeader>::new();
        
        match req.path.path.as_str() {
            "/" => {
                headers.insert("Content-Type".to_string(), KnownHeader::ContentType(HttpContentType::TextHtml));
                headers.insert("Content-Length".to_string(), KnownHeader::ContentLength("<h1>Welcome to the Rust HTTP Server!</h1>".bytes().len()));
                return HttpResponse {
                    version: req.version,
                    status_code: HttpStatusCode::OK,
                    headers: headers,
                    body: Some("<h1>Welcome to the Rust HTTP Server!</h1>".to_string()),
                }
            },
            "/json" => {
                let json_str = r#"
                {
                    "name": "John Doe",
                    "age": 30,
                    "is_student": false,        
                    "courses": ["Math", "Science", "History"],
                    "address": {
                        "street": "123 Main St",
                        "city": "Anytown",
                        "zip": "12345"
                    }
                }
                "#;

                headers.insert("Content-Type".to_string(), KnownHeader::ContentType(HttpContentType::ApplicationJson));
                headers.insert("Content-Length".to_string(), KnownHeader::ContentLength(json_str.bytes().len()));
                
                return HttpResponse {
                    version: req.version,
                    status_code: HttpStatusCode::OK,
                    headers: headers,
                    body: Some(json_str.to_string()),
                }
            },
            _ => {

                headers.insert("Content-Type".to_string(), KnownHeader::ContentType(HttpContentType::TextHtml));
                headers.insert("Content-Length".to_string(), KnownHeader::ContentLength("<h1>404 Not Found</h1>".bytes().len()));
                return HttpResponse {
                    version: req.version,
                    status_code: HttpStatusCode::NotFound,
                    headers: headers,
                    body: Some("<h1>404 Not Found</h1>".to_string()),
                }
            }
        }
    });

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let cln = platform.clone();
        std::thread::spawn(move || {
            cln.handle_request(stream);
        });
    }
}

fn http_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();

        let mut buf = String::new();
        let mut buf_reader = BufReader::new(&stream);

        println!("Request: {buf:#?}");

        let json = r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Sample JSON</title>
        </head>
        <body>
            <h1>Sample JSON Data</h1>
            <pre>
{
    "name": "John Doe",
    "age": 30,
    "is_student": false,        
    "courses": ["Math", "Science", "History"],
    "address": {
        "street": "123 Main St",
        "city": "Anytown",
        "zip": "12345"
    }
}
            </pre>
        </body>
        </html>
        "#;

        let response 
        
            = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
            json.bytes().len(),
            json
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        break;
    }
}

fn json_testing() {
        let json_str = r#"
    {
        "name": "John Doe",
        "age": 30,
        "is_student": false,
        "courses": ["Math", "Science", "History"],
        "address": {
            "street": "123 Main St",
            "city": "Anytown",
            "zip": "12345"
        }
    }
    "#;

    let parse_result = parse_json(json_str);
    match parse_result {
        Ok(json) => {
            let person = Person::from_json(&json);
            println!("Name: {}", person.name);
            println!("Age: {}", person.age);
            println!("Is Student: {}", person.is_student);
            println!("Courses: {:?}", person.courses);
            println!("Address: {}, {}, {}", person.address.street, person.address.city, person.address.zip);
        },
        Err(e) => {
            println!("Failed to parse JSON: {:?}", e);
        }
    }
}

struct Person {
    name: String,
    age: i32,
    is_student: bool,
    courses: Vec<String>,
    address: Address,
}

impl FromJson for Person {
    fn from_json(json: &JsonType) -> Self {
        match json {
            JsonType::Object(obj) => {
                let name = if let Some(JsonType::String(s)) = obj.get("name") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let age = if let Some(JsonType::Number(n)) = obj.get("age") {
                    *n as i32
                } else {
                    0
                };

                let is_student = if let Some(JsonType::Boolean(b)) = obj.get("is_student") {
                    *b
                } else {
                    false
                };

                let courses = if let Some(JsonType::Array(arr)) = obj.get("courses") {
                    arr.iter().filter_map(|item| {
                        if let JsonType::String(s) = item {
                            Some(s.clone())
                        } else {
                            None
                        }
                    }).collect()
                } else {
                    vec![]
                };

                let address = if let Some(addr_json) = obj.get("address") {
                    Address::from_json(addr_json.clone())
                } else {
                    Address {
                        street: "".to_string(),
                        city: "".to_string(),
                        zip: "".to_string(),
                    }
                };

                Person {
                    name,
                    age,
                    is_student,
                    courses,
                    address,
                }
            },
            _ => panic!("Expected a JSON object"),
        }
    }
}

struct Address {
    street: String,
    city: String,
    zip: String,
}

impl FromJson for Address {
    fn from_json(json: &JsonType) -> Self {
        match json {
            JsonType::Object(obj) => {
                let street = if let Some(JsonType::String(s)) = obj.get("street") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let city = if let Some(JsonType::String(s)) = obj.get("city") {
                    s.clone()
                } else {
                    "".to_string()
                };

                let zip = if let Some(JsonType::String(s)) = obj.get("zip") {
                    s.clone()
                } else {
                    "".to_string()
                };

                Address {
                    street,
                    city,
                    zip,
                }
            },
            _ => panic!("Expected a JSON object"),
        }
    }
}