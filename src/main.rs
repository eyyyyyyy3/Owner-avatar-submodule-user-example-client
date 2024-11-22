mod proto {
    tonic::include_proto!("example.v1");
}

use std::error::Error;

use proto::example_client::ExampleClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://127.0.0.1:4000";

    let mut example_server = ExampleClient::connect(url).await?;

    let server_response =
        example_server.say_something(tonic::Request::new(proto::SaySomethingRequest::default()));

    let a = server_response.await?;
    println!("The server said: {}", a.get_ref().answer);
    Ok(())
}
