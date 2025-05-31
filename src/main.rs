
use bytes::Bytes;
use socketioxide::{
    extract::{AckSender, Data, SocketRef},
    SocketIo,
};
use serde_json::Value;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::{Any, CorsLayer};

async fn on_connect(socket: SocketRef, Data(data): Data<serde_json::Value>) {
    info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
    socket.emit("auth", &data).ok();
    socket.on("server-volatile-broadcast", async |socket:SocketRef, Data::<Value>(data)| {
        info!("{} send to room id + data {:?}", socket.id,  data);
      //  socket.emit("client-broadcast", &data).ok();
    });
}

#[tokio::main] 
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let cors = CorsLayer::new().allow_origin(Any);

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);
    let app = axum::Router::new()
    .layer(layer)
    .layer(cors);

    info!("start server");
    

    let listener = tokio::net::TcpListener::bind("localhost:5000").await.unwrap();
    axum::serve(listener,app).await.unwrap();

    Ok(())
}