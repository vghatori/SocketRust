
use bytes::Bytes;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};

use serde_json::{Value};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower_http::cors::{Any, CorsLayer};



async fn on_connect(socket: SocketRef) {
   // info!(ns = socket.ns(), ?socket.id, "Socket.IO connected");
   
    socket.emit("init-room", "getroom_id").ok();
    socket.on("join-room", async |socket: SocketRef, Data::<String>(roomid)| {
        info!("new join! : {}", socket.id);
        socket.join(roomid.clone());
    });

    socket.on("server-volatile-broadcast", async|socket: SocketRef, Data::<(String, Bytes, Bytes)>(data)| {
        //info!("send :{:?}", data.1.clone());
        socket.broadcast().to(data.0.clone()).emit("client-broadcast",&(data.1,data.2)).await.ok();
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