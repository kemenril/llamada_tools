use llamada_tools_common::{HttpRequest, HttpResponse, from_json_str};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use serde_json::json;
use anyhow::{anyhow, Result};

fn emit(res: HttpResponse) {
    println!("{}", json!(res));
}

fn tool_error(msg: &str) -> HttpResponse {
    HttpResponse {
        http_status_code: 0,
        http_status_message: String::new(),
        local_tool_status_message: msg.to_string(),
        headers: HashMap::new(),
        body: String::new(),
        file: String::new(),
    }
}

async fn run() -> Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let req = match from_json_str::<HttpRequest>(&buffer) {
        Ok(r) => r,
        Err(e) => {
            emit(tool_error(&format!("Failed to parse input: {}", e)));
            return Err(e.into());
        }
    };

    let method = match req.method.trim().to_uppercase().as_str() {
        "GET"    => reqwest::Method::GET,
        "POST"   => reqwest::Method::POST,
        "PUT"    => reqwest::Method::PUT,
        "DELETE" => reqwest::Method::DELETE,
        "PATCH"  => reqwest::Method::PATCH,
        "HEAD"   => reqwest::Method::HEAD,
        other => {
            emit(tool_error(&format!("Unsupported HTTP method: {}", other)));
            return Err(anyhow!("Unsupported HTTP method: {}", other));
        }
    };

    let client = reqwest::Client::new();
    let mut request = client.request(method, &req.url);

    for (key, value) in &req.headers {
        request = request.header(key, value);
    }
    if !req.body.is_empty() {
        request = request.body(req.body.clone());
    }

    let response = match request.send().await {
        Ok(r) => r,
        Err(e) => {
            emit(tool_error(&format!("HTTP request failed: {}", e)));
            return Err(anyhow!("HTTP request failed: {}", e));
        }
    };

    let status_code = response.status().as_u16();
    let status_message = response.status().canonical_reason()
        .unwrap_or("Unknown")
        .to_string();

    // Apparently the call to .bytes() down there actually consumes the
    // response object, so this has to be done before that.
    let resp_headers: HashMap<String, String> = response
        .headers()
        .iter()
        .filter_map(|(k, v)| {
            v.to_str().ok().map(|v| (k.to_string(), v.to_string()))
        })
        .collect();

    let body_bytes = match response.bytes().await {
        Ok(b) => b,
        Err(e) => {
            emit(tool_error(&format!(
                "Request succeeded (status {}) but reading response body failed: {}",
                status_code, e
            )));
            return Err(anyhow!("Failed to read response body: {}", e));
        }
    };

    let mode = req.output_mode.trim();

    let res = if mode == "file" && !req.output_file.is_empty() {
        match fs::write(&req.output_file, &body_bytes) {
            Ok(_) => HttpResponse {
                http_status_code: status_code,
                http_status_message: status_message,
                local_tool_status_message: format!(
                    "Body written to file: {}", req.output_file
                ),
                headers: resp_headers,
                body: String::new(),
                file: req.output_file.clone(),
            },
            Err(e) => HttpResponse {
                http_status_code: status_code,
                http_status_message: status_message,
                local_tool_status_message: format!(
                    "Could not write to '{}' ({}); falling back to immediate mode",
                    req.output_file, e
                ),
                headers: resp_headers,
                body: String::from_utf8_lossy(&body_bytes).to_string(),
                file: String::new(),
            },
        }
    } else {
        HttpResponse {
            http_status_code: status_code,
            http_status_message: status_message,
            local_tool_status_message: if mode == "immediate" {
                "Request completed".to_string()
            } else {
                format!("Invalid output mode given: '{}'. Completed in immediate mode.", mode)
            },
            headers: resp_headers,
            body: String::from_utf8_lossy(&body_bytes).to_string(),
            file: String::new(),
        }
    };

    emit(res);
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    if run().await.is_err() {
        std::process::exit(1);
    }
}

