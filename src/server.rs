use project_restful::internal::{
    account::{
        store::DBClient, 
        self, 
        authorization,
    },
};
use tokio_postgres::NoTls;
use tonic::transport::Server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect("postgresql://postgres:postgres@localhost:5432/postgres", NoTls).await?;

    let account_db_client = DBClient::new(client);
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let addr = "0.0.0.0:40130".parse()?;
    let authorization_service = account::AuthorizationService::new(account_db_client);
    tokio::spawn(async move {
        Server::builder()
            .add_service(authorization::authorization_server::AuthorizationServer::new(authorization_service))
            .serve(addr)
            .await
            .unwrap();
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}



