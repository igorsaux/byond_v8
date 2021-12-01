use server::server::Server;

#[tokio::main]
async fn main() {
  let args = std::env::args().collect::<Vec<String>>();

  let server_name = args
    .get(1)
    .expect("Pass the server name in args.");

  let mut client = Server::new(server_name);
  client.run().await;
}
