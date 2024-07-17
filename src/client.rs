use tonic::transport::{channel, Channel};

mod parse;
mod actions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let action = parse::parse();

    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    send_action(channel, action).await;

    Ok(())
}

async fn send_action(channel: Channel, action: actions::ActionRequest) {
    let mut client = actions::action_client::ActionClient::new(channel);

    let request = tonic::Request::new(action);

    let response = client.schedule(request).await.unwrap();

    println!("RESPONSE={:?}", response);
}