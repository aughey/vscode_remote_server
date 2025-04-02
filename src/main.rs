use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde_json::json;
use std::time::{Duration, Instant};

struct WebSocketConnection {
    start_time: Instant,
}

impl Actor for WebSocketConnection {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketConnection {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Pong(_)) => {
                // Check if 5 seconds have elapsed since connection
                if self.start_time.elapsed() >= Duration::from_secs(5) {
                    let command = json!({
                        "command": {
                            "type": "barf"
                        }
                    });
                    ctx.text(command.to_string());
                }
            }
            Ok(ws::Message::Text(_)) => (),
            Ok(ws::Message::Binary(_)) => (),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn ws_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(
        WebSocketConnection {
            start_time: Instant::now(),
        },
        &req,
        stream,
    )?;
    Ok(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting WebSocket server on http://127.0.0.1:8080/ws");

    HttpServer::new(|| App::new().route("/ws", web::get().to(ws_index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
