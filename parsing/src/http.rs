use std::collections::HashMap;

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT
}

pub enum HttpVersion {
    HTTP10,
    HTTP11,
    HTTP20
}

pub enum HttpContentType {
    TextHtml,
    ApplicationJson,
    ApplicationXml,
    TextPlain,
    MultipartFormData,
    ApplicationXWwwFormUrlencoded,
    EventStream,
    Other(String)
}

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
    ServiceUnavailable = 503
}

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
    Other(String) // (header name, header value)
}

pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
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
}

type HttpHandler = fn(HttpRequest) -> Result<HttpResponse, HttpRequestError>;


pub fn validate_http_request(request: &HttpRequest) -> Result<(), HttpRequestError> {
    // TODO: Implement read validation logic 
    Ok(())
}

pub fn write_http_request(request: HttpRequest) -> Result<(), HttpRequestError> {
    return Ok(());
}

pub fn write_http_response(response: HttpResponse) -> Result<(), HttpRequestError> {
    return Ok(());
}

pub fn read_http_response(input: &str) -> Result<HttpResponse, HttpRequestError> {
    Ok(
        HttpResponse {
        version: HttpVersion::HTTP11, 
        status_code: HttpStatusCode::OK, 
        headers: HashMap::new(), 
        body: None 
    })
}

pub fn read_http(input: &str) -> Result<HttpRequest, HttpRequestError> {
    // TODO: Implement a real HTTP request parser
    Ok(
        HttpRequest {
        method: HttpMethod::GET,
        path: String::new(), 
        version: HttpVersion::HTTP11, 
        headers: HashMap::new(), 
        body: None 
    })
}