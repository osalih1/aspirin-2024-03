use aspirin_eats::db::AspirinEatsDb;
use aspirin_eats::error::AspirinEatsError;
use aspirin_eats::food::{Order, OrderRequest};
use aspirin_eats::http::{HttpRequest, HttpResponse};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::str::FromStr;

fn main() -> Result<(), AspirinEatsError> {
    const DB_PATH: &str = "aspirin_eats.db";
    let db = AspirinEatsDb::from_path(DB_PATH)?;

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
                continue;
            }
        };

        let mut buffer = [0; 1024];
        let bytes_read = match stream.read(&mut buffer) {
            Ok(br) => br,
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                continue;
            }
        };
        let request_str = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        let request = match HttpRequest::from_str(&request_str) {
            Ok(req) => req,
            Err(_) => {
                let response: HttpResponse = AspirinEatsError::InvalidRequest.into();
                stream.write_all(response.to_string().as_bytes())?;
                continue;
            }
        };

        let response = handle_request(request, &db);
        if let Err(e) = stream.write_all(response.to_string().as_bytes()) {
            eprintln!("Failed to write to stream: {}", e);
        }
    }

    Ok(())
}

fn handle_request(request: HttpRequest, db: &AspirinEatsDb) -> HttpResponse {
    match (request.method.as_deref(), request.path.as_deref()) {
        (Some("GET"), Some("/")) | (Some("GET"), None) => {
            HttpResponse::new(200, "OK", "Welcome to Aspirin Eats!")
        }
        (Some("GET"), Some("/orders")) => match db.get_all_orders() {
            Ok(orders) => {
                let body = serde_json::to_string(&orders).unwrap_or_else(|_| "[]".to_string());
                HttpResponse::new(200, "OK", &body)
            }
            Err(e) => AspirinEatsError::from(e).into(),
        },
        (Some("GET"), Some(path)) if path.starts_with("/orders/") => {
            let id_str = path.trim_start_matches("/orders/");
            match id_str.parse::<i64>() {
                Ok(id) => match db.get_order(id) {
                    Ok(Some(order)) => {
                        let body = serde_json::to_string(&order).unwrap_or_default();
                        HttpResponse::new(200, "OK", &body)
                    }
                    Ok(None) => HttpResponse::new(404, "Not Found", "Order not found"),
                    Err(e) => AspirinEatsError::from(e).into(),
                },
                Err(_) => HttpResponse::new(400, "Bad Request", "Invalid order ID"),
            }
        }
        (Some("POST"), Some("/orders")) => {
            if let Some(body) = request.body {
                match serde_json::from_str::<OrderRequest>(&body) {
                    Ok(order_request) => {
                        let order = Order::from(order_request);
                        match db.add_order(order) {
                            Ok(id) => {
                                let added_order = db.get_order(id).unwrap().unwrap();
                                let body = serde_json::to_string(&added_order).unwrap_or_default();
                                HttpResponse::new(201, "Created", &body)
                            }
                            Err(e) => AspirinEatsError::from(e).into(),
                        }
                    }
                    Err(_) => HttpResponse::new(400, "Bad Request", "Invalid order data"),
                }
            } else {
                HttpResponse::new(400, "Bad Request", "Missing request body")
            }
        }
        (Some("DELETE"), Some("/orders")) => match db.reset_orders() {
            Ok(_) => HttpResponse::new(200, "OK", "All orders deleted"),
            Err(e) => AspirinEatsError::from(e).into(),
        },
        (Some("DELETE"), Some(path)) if path.starts_with("/orders/") => {
            let id_str = path.trim_start_matches("/orders/");
            match id_str.parse::<i64>() {
                Ok(id) => match db.remove_order(id) {
                    Ok(_) => HttpResponse::new(200, "OK", "Order deleted"),
                    Err(e) => AspirinEatsError::from(e).into(),
                },
                Err(_) => HttpResponse::new(400, "Bad Request", "Invalid order ID"),
            }
        }
        (Some(_), Some(_)) => HttpResponse::new(405, "Method Not Allowed", "Method not allowed"),
        _ => HttpResponse::new(404, "Not Found", "Resource not found"),
    }
}
