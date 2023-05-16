use tokio;
use tokio_tungstenite;
use subxt::{OnlineClient, PolkadotConfig};
use tokio_tungstenite::{connect_async, tungstenite::Message, WebSocketStream};

pub use connect::*;

pub mod connect {
    use subxt::client::OnlineClientT;

    use super::*;

    pub async fn connect_to_rpc() ->Result<Box<dyn OnlineClientT>,Box<dyn std::error::Error>>{
        let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8000").await.unwrap();
        Ok(api)
    }

    pub async fn connect_to_ws() ->Result<WebSocketStream<S>,Box<dyn std::error::Error>>{
        let (mut ws_stream,response) = connect_async("ws://127.0.0.1:8000").await.unwrap();
        Ok(ws_stream)
    }


        //  // Reading the back stream
        //  while let Some(Ok(message)) = ws_stream.next().await{
        //     match message {
        //         Message::Text(text) => {
        //             println!("{:?}",text)
        //         },
        //         _=> println!("Some other message")
        //     }
        // }
   
}
