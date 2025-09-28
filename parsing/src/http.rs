use std::{
    collections::HashMap,
    io::{BufRead, Read, Write},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpVersion {
    HTTP10,
    HTTP11,
    HTTP20,
}

impl HttpVersion {
    fn to_str(&self) -> &str {
        match self {
            HttpVersion::HTTP10 => "HTTP/1.0",
            HttpVersion::HTTP11 => "HTTP/1.1",
            HttpVersion::HTTP20 => "HTTP/2.0",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpContentType {
    TextHtml,
    ApplicationJson,
    ApplicationXml,
    TextPlain,
    MultipartFormData,
    ApplicationXWwwFormUrlencoded,
    EventStream,
    Other(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpStatusCode {
    OK = 200,
    Created = 201,
    Accepted = 202,
    NoContent = 204,
    MovedPermanently = 301,
    Found = 302,
    NotModified = 304,
    BadRequest = 400,
    Unauthorized = 401,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
}

impl HttpStatusCode {
    fn to_str(&self) -> &str {
        match self {
            HttpStatusCode::OK => "200",
            HttpStatusCode::Created => "201",
            HttpStatusCode::Accepted => "202",
            HttpStatusCode::NoContent => "204",
            HttpStatusCode::MovedPermanently => "301",
            HttpStatusCode::Found => "302",
            HttpStatusCode::NotModified => "304",
            HttpStatusCode::BadRequest => "400",
            HttpStatusCode::Unauthorized => "401",
            HttpStatusCode::Forbidden => "403",
            HttpStatusCode::NotFound => "404",
            HttpStatusCode::MethodNotAllowed => "405",
            HttpStatusCode::InternalServerError => "500",
            HttpStatusCode::NotImplemented => "501",
            HttpStatusCode::BadGateway => "502",
            HttpStatusCode::ServiceUnavailable => "503",
        }
    }
}

impl HttpStatusCode {
    fn status_text(&self) -> &str {
        match self {
            HttpStatusCode::OK => "OK",
            HttpStatusCode::Created => "Created",
            HttpStatusCode::Accepted => "Accepted",
            HttpStatusCode::NoContent => "No Content",
            HttpStatusCode::MovedPermanently => "Moved Permanently",
            HttpStatusCode::Found => "Found",
            HttpStatusCode::NotModified => "Not Modified",
            HttpStatusCode::BadRequest => "Bad Request",
            HttpStatusCode::Unauthorized => "Unauthorized",
            HttpStatusCode::Forbidden => "Forbidden",
            HttpStatusCode::NotFound => "Not Found",
            HttpStatusCode::MethodNotAllowed => "Method Not Allowed",
            HttpStatusCode::InternalServerError => "Internal Server Error",
            HttpStatusCode::NotImplemented => "Not Implemented",
            HttpStatusCode::BadGateway => "Bad Gateway",
            HttpStatusCode::ServiceUnavailable => "Service Unavailable",
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum KnownHeader {
    ContentType(HttpContentType),
    ContentLength(usize),
    UserAgent(String),
    Accept(String),
    Host(String),
    Authorization(String),
    CacheControl(String),
    Connection(String),
    Cookie(String),
    Referer(String),
    Other(String), // (header name, header value)
}

impl KnownHeader {
    fn from_str(header_name: &str, header_value: &str) -> KnownHeader {
        match header_name.to_lowercase().as_str() {
            "content-type" => KnownHeader::ContentType(HttpContentType::from_str(header_value)),
            "content-length" => {
                if let Ok(length) = header_value.parse::<usize>() {
                    KnownHeader::ContentLength(length)
                } else {
                    KnownHeader::Other(header_value.to_string())
                }
            }
            "user-agent" => KnownHeader::UserAgent(header_value.to_string()),
            "accept" => KnownHeader::Accept(header_value.to_string()),
            "host" => KnownHeader::Host(header_value.to_string()),
            "authorization" => KnownHeader::Authorization(header_value.to_string()),
            "cache-control" => KnownHeader::CacheControl(header_value.to_string()),
            "connection" => KnownHeader::Connection(header_value.to_string()),
            "cookie" => KnownHeader::Cookie(header_value.to_string()),
            "referer" => KnownHeader::Referer(header_value.to_string()),
            _ => KnownHeader::Other(header_value.to_string()),
        }
    }
}

pub struct HttpPath {
    pub full_path: String,
    pub path: String,
    pub query: Option<HashMap<String, String>>,
    pub fragment: Option<String>,
}

impl HttpPath {
    fn from_str(path: &str) -> HttpPath {
        let mut full_path = path.to_string();
        let mut path_only = path.to_string();
        let mut query: Option<HashMap<String, String>> = None;
        let mut fragment: Option<String> = None;

        if let Some(hash_index) = full_path.find('#') {
            fragment = Some(full_path[hash_index + 1..].to_string());
            full_path = full_path[..hash_index].to_string();
        }

        if let Some(question_index) = full_path.find('?') {
            let query_str = &full_path[question_index + 1..];
            path_only = full_path[..question_index].to_string();

            let mut query_map = HashMap::new();
            for pair in query_str.split('&') {
                let mut key_value = pair.splitn(2, '=');
                if let Some(key) = key_value.next() {
                    let value = key_value.next().unwrap_or("");
                    query_map.insert(key.to_string(), value.to_string());
                }
            }
            query = Some(query_map);
        } else {
            path_only = full_path.clone();
        }

        HttpPath {
            full_path: full_path,
            path: path_only,
            query,
            fragment,
        }
    }
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: HttpPath,
    pub version: HttpVersion,
    pub headers: HashMap<String, KnownHeader>,
    pub body: Option<String>,
}

pub struct HttpResponse {
    pub version: HttpVersion,
    pub status_code: HttpStatusCode,
    pub headers: HashMap<String, KnownHeader>,
    pub body: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum HttpRequestError {
    InvalidRequest(String),
    InvalidHeader(String),
    InvalidMethod(String),
    InvalidVersion(String),
}

impl HttpMethod {
    fn from_str(method: &str) -> Result<HttpMethod, HttpRequestError> {
        match method {
            "GET" => Ok(HttpMethod::GET),
            "POST" => Ok(HttpMethod::POST),
            "PUT" => Ok(HttpMethod::PUT),
            "DELETE" => Ok(HttpMethod::DELETE),
            "HEAD" => Ok(HttpMethod::HEAD),
            "OPTIONS" => Ok(HttpMethod::OPTIONS),
            "PATCH" => Ok(HttpMethod::PATCH),
            "TRACE" => Ok(HttpMethod::TRACE),
            "CONNECT" => Ok(HttpMethod::CONNECT),
            _ => Err(HttpRequestError::InvalidMethod(method.to_string())),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::PATCH => "PATCH",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::CONNECT => "CONNECT",
        }
    }
}

impl HttpContentType {
    fn from_str(content_type: &str) -> HttpContentType {
        match content_type {
            "text/html" => HttpContentType::TextHtml,
            "application/json" => HttpContentType::ApplicationJson,
            "application/xml" => HttpContentType::ApplicationXml,
            "text/plain" => HttpContentType::TextPlain,
            "multipart/form-data" => HttpContentType::MultipartFormData,
            "application/x-www-form-urlencoded" => HttpContentType::ApplicationXWwwFormUrlencoded,
            "text/event-stream" => HttpContentType::EventStream,
            other => HttpContentType::Other(other.to_string()),
        }
    }

    fn to_str(&self) -> &str {
        match self {
            HttpContentType::TextHtml => "text/html",
            HttpContentType::ApplicationJson => "application/json",
            HttpContentType::ApplicationXml => "application/xml",
            HttpContentType::TextPlain => "text/plain",
            HttpContentType::MultipartFormData => "multipart/form-data",
            HttpContentType::ApplicationXWwwFormUrlencoded => "application/x-www-form-urlencoded",
            HttpContentType::EventStream => "text/event-stream",
            HttpContentType::Other(s) => s,
        }
    }
}

type HttpHandler = fn(HttpRequest) -> HttpResponse;

pub fn write_http_request(request: HttpRequest) -> Result<(), HttpRequestError> {
    return Ok(());
}

pub fn write_http_response(response: HttpResponse) -> Result<String, HttpRequestError> {
    let mut output = format!(
        "{} {} {}\r\n",
        response.version.to_str(),
        response.status_code.to_str(),
        response.status_code.status_text()
    );

    for (header_name, header_value) in response.headers.iter() {
        let header_line = match header_value {
            KnownHeader::ContentType(ct) => format!("{}: {}\r\n", header_name, ct.to_str()),
            KnownHeader::ContentLength(len) => format!("{}: {}\r\n", header_name, len),
            KnownHeader::UserAgent(ua) => format!("{}: {}\r\n", header_name, ua),
            KnownHeader::Accept(acc) => format!("{}: {}\r\n", header_name, acc),
            KnownHeader::Host(host) => format!("{}: {}\r\n", header_name, host),
            KnownHeader::Authorization(auth) => format!("{}: {}\r\n", header_name, auth),
            KnownHeader::CacheControl(cc) => format!("{}: {}\r\n", header_name, cc),
            KnownHeader::Connection(conn) => format!("{}: {}\r\n", header_name, conn),
            KnownHeader::Cookie(cookie) => format!("{}: {}\r\n", header_name, cookie),
            KnownHeader::Referer(referer) => format!("{}: {}\r\n", header_name, referer),
            KnownHeader::Other(value) => format!("{}: {}\r\n", header_name, value),
        };
        output.push_str(&header_line);
    }

    output.push_str("\r\n");

    if let Some(body) = response.body {
        output.push_str(body.as_str());
    }

    return Ok(output);
}


#[derive(Clone)]
pub struct HttpPlatform {
    pub app: HttpHandler,
}

impl HttpPlatform {
    pub fn new(app: HttpHandler) -> HttpPlatform {
        HttpPlatform { app }
    }

    pub fn handle_request(&self, mut stream: std::net::TcpStream) {
        let mut buf = [0; 8024];

        loop {
            match stream.read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }
                    let buf = String::from_utf8(buf[..n].to_vec()).unwrap();
                    match read_http_request(buf.as_str()) {
                        Ok(request) => {
                            let response = (self.app)(request);
                            let response_str = write_http_response(response).unwrap();
                            stream.write(response_str.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }
                        Err(e) => {
                            let error_response = HttpResponse {
                                version: HttpVersion::HTTP11,
                                status_code: HttpStatusCode::BadRequest,
                                headers: HashMap::new(),
                                body: None,
                            };

                            let response_str = write_http_response(error_response).unwrap();
                            stream.write(response_str.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }
                    }
                }
                Err(_) => {
                    return;
                }
            }
        }
    }
}

pub fn read_http_response(input: &str) -> Result<HttpResponse, HttpRequestError> {
    Ok(HttpResponse {
        version: HttpVersion::HTTP11,
        status_code: HttpStatusCode::OK,
        headers: HashMap::new(),
        body: None,
    })
}

enum ParserState {
    RequestLine,
    Headers,
    Body,
}

pub fn read_http_request(mut input: &str) -> Result<HttpRequest, HttpRequestError> {
    let mut state = ParserState::RequestLine;
    let mut method = HttpMethod::GET;
    let mut path = HttpPath::from_str("/");
    let mut version = HttpVersion::HTTP11;
    let mut headers: HashMap<String, KnownHeader> = HashMap::new();
    let mut body: Option<String> = None;

    input = input.trim_start();
    for line in input.lines() {
        match state {
            ParserState::RequestLine => {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.len() != 3 {
                    return Err(HttpRequestError::InvalidRequest(
                        "Malformed request line".to_string(),
                    ));
                }

                method = HttpMethod::from_str(parts[0])?;
                path = HttpPath::from_str(parts[1]);
                version = match parts[2] {
                    "HTTP/1.0" => HttpVersion::HTTP10,
                    "HTTP/1.1" => HttpVersion::HTTP11,
                    "HTTP/2.0" => HttpVersion::HTTP20,
                    _ => return Err(HttpRequestError::InvalidVersion(parts[2].to_string())),
                };

                state = ParserState::Headers;
            }
            ParserState::Headers => {
                if line.is_empty() {
                    state = ParserState::Body;
                    continue;
                }

                let parts: Vec<&str> = line.splitn(2, ':').collect();
                if parts.len() != 2 {
                    return Err(HttpRequestError::InvalidHeader(
                        "Malformed header line".to_string(),
                    ));
                }

                let header_name = parts[0].trim();
                let header_value = parts[1].trim();

                headers.insert(
                    header_name.to_string(),
                    KnownHeader::from_str(header_name, header_value),
                );
            }
            ParserState::Body => match body {
                Some(ref mut b) => {
                    b.push_str(format!("\r\n{}", line.trim()).as_str());
                }
                None => {
                    body = Some(line.trim().to_string());
                }
            },
            _ => {}
        }
    }

    Ok(HttpRequest {
        method: method,
        path: path,
        version: version,
        headers: headers,
        body: body,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_http_get_request() {
        let request_str = "GET / HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path.full_path, "/");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(request.body, None);
    }

    #[test]
    fn read_http_get_request_with_query_parameters() {
        let request_str = "GET /search?q=rust+language HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestAgent/1.0\r\n\r\n";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path.full_path, "/search?q=rust+language");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&KnownHeader::UserAgent("TestAgent/1.0".to_string()))
        );
        assert_eq!(request.body, None);
    }

    #[test]
    fn read_http_get_request_with_fragment() {
        let request_str = "GET /page#section HTTP/1.1\r\nHost: example.com\r\n\r\n";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path.full_path, "/page#section");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(request.body, None);
    }

    #[test]
    fn read_http_get_request_with_multiple_headers() {
        let request_str = "GET / HTTP/1.1\r\nHost: example.com\r\nUser-Agent: TestAgent/1.0\r\nAccept: text/html\r\n\r\n";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.path.full_path, "/");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(
            request.headers.get("User-Agent"),
            Some(&KnownHeader::UserAgent("TestAgent/1.0".to_string()))
        );
        assert_eq!(
            request.headers.get("Accept"),
            Some(&KnownHeader::Accept("text/html".to_string()))
        );
        assert_eq!(request.body, None);
    }

    #[test]
    fn read_http_post_request() {
        let request_str = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\nContent-Length: 18\r\n\r\n{\"key\":\"value\"}";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path.full_path, "/submit");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&KnownHeader::ContentType(HttpContentType::ApplicationJson))
        );
        assert_eq!(
            request.headers.get("Content-Length"),
            Some(&KnownHeader::ContentLength(18))
        );
        assert_eq!(request.body, Some("{\"key\":\"value\"}".to_string()));
    }

    #[test]
    fn read_http_post_request_with_multiline_body() {
        let request_str = "POST /submit HTTP/1.1\r\nHost: example.com\r\nContent-Type: application/json\r\nContent-Length: 36\r\n\r\n{\r\n\"key1\":\"value1\",\r\n\"key2\":\"value2\"\r\n}";
        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path.full_path, "/submit");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&KnownHeader::ContentType(HttpContentType::ApplicationJson))
        );
        assert_eq!(
            request.headers.get("Content-Length"),
            Some(&KnownHeader::ContentLength(36))
        );
        assert_eq!(
            request.body,
            Some("{\r\n\"key1\":\"value1\",\r\n\"key2\":\"value2\"\r\n}".to_string())
        );
    }

    #[test]
    fn read_http_post_request_with_body_spaces() {
        let request_str = r#"
        POST /submit HTTP/1.1
        Host: example.com
        Content-Type: application/json
        Content-Length: 36

        {
            "key1": "value1",
            "key2": "value2"
        }
        "#;

        let request = read_http_request(request_str).unwrap();

        assert_eq!(request.method, HttpMethod::POST);
        assert_eq!(request.path.full_path, "/submit");
        assert_eq!(request.version, HttpVersion::HTTP11);
        assert_eq!(
            request.headers.get("Host"),
            Some(&KnownHeader::Host("example.com".to_string()))
        );
        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&KnownHeader::ContentType(HttpContentType::ApplicationJson))
        );
        assert_eq!(
            request.headers.get("Content-Length"),
            Some(&KnownHeader::ContentLength(36))
        );
        assert_eq!(
            request.body,
            Some("{\r\n\"key1\": \"value1\",\r\n\"key2\": \"value2\"\r\n}\r\n".to_string())
        );
    }

    #[test]
    fn write_http_response_test() {
        let response = HttpResponse {
            version: HttpVersion::HTTP11,
            status_code: HttpStatusCode::OK,
            headers: {
                let mut headers = HashMap::new();
                headers.insert(
                    "Content-Type".to_string(),
                    KnownHeader::ContentType(HttpContentType::TextHtml),
                );
                headers.insert("Content-Length".to_string(), KnownHeader::ContentLength(13));
                headers
            },
            body: Some("<h1>Hello</h1>".to_string()),
        };

        let response_str = write_http_response(response).unwrap();
        let expected_response_str = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\nContent-Type: text/html\r\n\r\n<h1>Hello</h1>";

        assert_eq!(response_str, expected_response_str);
    }
}
