use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::HashMap;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    // unwrap()はResult型を取り出す→エラーハンドリングの簡略化→サーバーの起動に失敗した場合にプログラムが終了
    println!("Server is running on http://127.0.0.1:7878");
    for stream in listner.incoming() {
        let stream = stream.unwrap() ;
        handle_connection(stream)       
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request_str = String::from_utf8_lossy(&buffer).to_string();
    let (method, path, headers, body) = parse_request(&request_str);

    if path.starts_with("/api") {
        handle_api_request(method, path, &headers, &body, stream);
        return;
    }

    let file_path = format!("static{}", if path == "/" {"/index.html"} else {&path});
    let response = match fs::read(&file_path) {
        Ok(contents) =>{
            let content_type = get_content_type(&file_path);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                content_type,
                contents.len(),
                String::from_utf8_lossy(&contents)
            )
        },
        Err(_) =>{
            let not_found = "404 Not Found";
            format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                not_found.len(),
                not_found
            )
        }
    };
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_api_request(method: &str, path: &str, headers: &HashMap<String, String>, body: &String, mut stream: TcpStream) {
    match (method, path) {
        ("POST", "/api/login") => handle_login(headers, body, stream),
        ("GET", "/api/hello") => {
            let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"message\": \"Hello API!\"}";
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        },
        _ => {
            let not_found = "404 API Endpoint Not Found";
            let response = format!(
                "HTTP/1.1 404 NOT FOUND\r\nContent-Length: {}\r\n\r\n{}",
                not_found.len(),
                not_found
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }
}

fn handle_login(headers: &HashMap<String, String>, body: &String, mut stream: TcpStream) {
    // 簡易的なJSONパース
    let mut username = String::new();
    let mut password = String::new();
    
    // 非常に簡易的なJSONパーサー（実際のアプリではserde_jsonなどのライブラリを使用）
    if body.contains("username") && body.contains("password") {
        // username抽出
        if let Some(start) = body.find("\"username\":") {
            let start = start + "\"username\":".len();
            if let Some(quote_start) = body[start..].find('"') {
                let start = start + quote_start + 1;
                if let Some(quote_end) = body[start..].find('"') {
                    username = body[start..(start + quote_end)].to_string();
                }
            }
        }
        
        // password抽出
        if let Some(start) = body.find("\"password\":") {
            let start = start + "\"password\":".len();
            if let Some(quote_start) = body[start..].find('"') {
                let start = start + quote_start + 1;
                if let Some(quote_end) = body[start..].find('"') {
                    password = body[start..(start + quote_end)].to_string();
                }
            }
        }
    }
    
    // ログの出力
    println!("ログイン試行: ユーザー名={}, パスワード={}", username, password);
    
    // 認証処理（実際のアプリでは安全な認証処理を実装する）
    let response = if !username.is_empty() && !password.is_empty() {
        // 実際のアプリではデータベースなどでユーザー情報を検証
        let token = format!("user_{}_token", username);
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{{\"status\": \"success\", \"token\": \"{}\"}}",
            token
        )
    } else {
        format!("HTTP/1.1 400 Bad Request\r\nContent-Type: application/json\r\n\r\n{{\"status\": \"error\", \"message\": \"ユーザー名とパスワードが必要です\"}}")
    };
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parse_request(request: &str) -> (&str, &str, HashMap<String, String>, String) {
    let mut lines = request.lines();
    
    // リクエストライン（最初の行）をパース
    let request_line = lines.next().unwrap_or("");
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    let method = if parts.len() > 0 { parts[0] } else { "" };
    let path = if parts.len() > 1 { parts[1] } else { "/" };
    
    // ヘッダーのパース
    let mut headers = HashMap::new();
    let mut body_start = 0;
    
    for (i, line) in request.lines().enumerate() {
        if i == 0 {
            continue; // リクエストラインはスキップ
        }
        
        if line.is_empty() {
            // 空行の後にボディが始まる
            body_start = request[..request.len()]
                .find("\r\n\r\n")
                .map(|pos| pos + 4)
                .or_else(|| request[..request.len()].find("\n\n").map(|pos| pos + 2))
                .unwrap_or(request.len());
            break;
        }
        
        if let Some(colon_pos) = line.find(':') {
            let key = line[..colon_pos].trim().to_string();
            let value = line[colon_pos + 1..].trim().to_string();
            headers.insert(key, value);
        }
    }
    
    let body = if body_start < request.len() {
        request[body_start..].to_string()
    } else {
        String::new()
    };
    
    (method, path, headers, body)
}

fn get_content_type(path: &str) -> &str {
    if path.ends_with(".html"){
        "text/html"
    }else if path.ends_with(".css") {
        "text/css"
    }else if path.ends_with(".js") {
        "text/javascript"
    }else if path.ends_with(".json") {
        "application/json"
    }else {
        "text/plain"
    }
}