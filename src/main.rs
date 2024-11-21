use chrono::{SecondsFormat, Utc};
use salvo::logging::Logger;
use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Message {
    status: String,
    message: String,
    timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiError {
    status: i32,
    timestamp: String,
    message: String,
    debug_message: String,
}

type AppResponse<T> = Result<T, ApiError>;

#[async_trait]
impl Writer for ApiError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let error = ApiError {
            status: 500,
            timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
            message: "Internal server error".to_owned(),
            debug_message: "Internal server error try after some time".to_string(),
        };
        res.render(Json(error));
    }
}

#[handler]
async fn hello_world(res: &mut Response) -> AppResponse<()> {
    let message = Message {
        status: "ok".to_string(),
        message: "working".to_string(),
        timestamp: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
    };
    res.render(Json(message));
    Ok(())
}

#[handler]
async fn get_candidates(res: &mut Response) -> AppResponse<()> {
    let candidates = vec![
        "Alice".to_string(),
        "Bob".to_string(),
        "Charlie".to_string(),
        "David".to_string(),
    ];
    res.render(Json(candidates));
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello_world);
    let create_candidate_route = Router::with_path("/candidates").get(get_candidates);
    let app_router = Router::new().push(router).push(create_candidate_route);
    let service = Service::new(app_router).hoop(Logger::new());
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(service).await;
}
