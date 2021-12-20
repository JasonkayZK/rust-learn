#[macro_export]
macro_rules! sender_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Database Manager failed to get {}! error: {}", $f, e);
                return Ok(Response::builder()
                    .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Something went wrong: {}", e)))
                    .unwrap());
            }
        }
    };
}

#[macro_export]
macro_rules! recv_failed {
    ($m: expr) => {
        match $m {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Database Manager returned error: {}", e);
                return Ok(Response::builder()
                    .status(hyper::StatusCode::NOT_FOUND)
                    .body(Body::from("Key does not exist"))
                    .unwrap());
            }
        }
    };
}

#[macro_export]
macro_rules! resp_failed {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {}
            Err(e) => tracing::error!("Resp failed for {}, error: {:?}", $f, e),
        }
    };
}

#[macro_export]
macro_rules! json_response {
    (body: $body:expr) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(serde_json::to_string($body).unwrap().into())
            .unwrap()
    };
    (status: $status:expr, body: $body:expr) => {
        Response::builder()
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .status($status)
            .body(serde_json::to_string($body).unwrap().into())
            .unwrap()
    };
    (error: $e:expr) => {
        json_response!(
            status: hyper::StatusCode::INTERNAL_SERVER_ERROR,
            body: &serde_json::json!({
                "error": $e.to_string(),
            }).to_string())
    };
}

#[macro_export]
macro_rules! sender_failed_json {
    ($m: expr, $f: tt) => {
        match $m {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Database Manager failed to get {}! error: {}", $f, e);
                return Ok(json_response!(error: e));
            }
        }
    };
}

#[macro_export]
macro_rules! recv_failed_json {
    ($m: expr, $status: expr) => {
        match $m {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Database Manager returned error: {}", e);
                return Ok(json_response!(
                        status: $status,
                        body: &e.to_string()))
            }
        }
    }
}
