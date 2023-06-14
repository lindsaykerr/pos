
pub const JSON_RESOURCE_NOT_FOUND: (u16, bool, &str, &str) = (404, false, "Resource Not Found", "HTTP/1.1 404 OK");
pub const JSON_BAD_REQUEST:(u16, bool, &str, &str) = (400,false, "Bad Request", "HTTP/1.1 400 OK");
pub const JSON_SUCCESS: (u16, bool, &str, &str)  = (200, true, "Success", "HTTP/1.1 200 OK");
pub const JSON_SERVER_ERROR: (u16, bool, &str, &str) = (500, false, "Internal Server Error", "HTTP/1.1 500 Internal Server Error");

pub fn standard_json_response(json_response: (u16, bool, &str, &str)) -> (String, String, String) {
    let response = json::object!{
        "status_code" => json_response.0,
        "success" => json_response.1,
        "message" => json_response.2,
    };
    (response.dump(), "application/json".to_string(), json_response.3.to_string())
}


pub const HTML_NOT_FOUND: (&str, &str) = ("404 Not Found", "HTTP/1.1 404 OK");
pub const HTML_BAD_REQUEST: (&str, &str) = ("400 Bad Request", "HTTP/1.1 400 OK");
pub const HTML_SERVER_ERROR: (&str, &str) = ("500 Internal Server Error", "HTTP/1.1 500 Internal Server Error");

pub fn standard_html_response(html_response: (&str, &str)) -> (String, String, String) {
    let response = format!("<p>{}</p>", html_response.0);
    (response, "text/html".to_string(), html_response.1.to_string())
}