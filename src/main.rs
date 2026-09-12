use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const CAT_IMAGE: &[u8] = include_bytes!("../assets/ragdoll-cat.png");

fn handle(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let size = stream.read(&mut buffer).unwrap_or(0);
    let request = String::from_utf8_lossy(&buffer[..size]);
    let path = request.split_whitespace().nth(1).unwrap_or("/");

    let (status, content_type, body): (&str, &str, &[u8]) = match path {
        "/cat.png" => ("200 OK", "image/png", CAT_IMAGE),
        "/" => (
            "200 OK",
            "text/html; charset=utf-8",
            r#"<!doctype html>
<html lang="zh-CN">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <title>布偶猫</title>
  <style>
    * { box-sizing: border-box; }
    body { margin: 0; min-height: 100vh; display: grid; place-items: center; background: #f4efe9; font-family: system-ui, sans-serif; color: #34313a; }
    main { width: min(920px, 92vw); padding: 32px; text-align: center; }
    h1 { margin: 0 0 20px; font-size: clamp(32px, 6vw, 64px); }
    img { display: block; width: 100%; border-radius: 28px; box-shadow: 0 24px 70px rgba(61, 51, 45, .2); }
  </style>
</head>
<body><main><h1>Hello, Ragdoll!</h1><img src="/cat.png" alt="一只蓝眼睛的布偶猫"></main></body>
</html>"#.as_bytes(),
        ),
        _ => ("404 Not Found", "text/plain; charset=utf-8", b"Not Found"),
    };

    let header = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("failed to bind port 3000");
    println!("Open http://localhost:3000");
    for stream in listener.incoming().flatten() {
        handle(stream);
    }
}
