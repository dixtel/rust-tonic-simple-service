use std::time::Duration;

use tokio::io::AsyncWriteExt;

mod service;

type Error = Result<(), Box<dyn std::error::Error>>;

struct Service {}

#[tonic::async_trait]
impl service::service_server::Service for Service {
    async fn add(
        &self,
        request: tonic::Request<service::AddRequest>,
    ) -> Result<tonic::Response<service::AddResponse>, tonic::Status> {
        Ok(tonic::Response::new(service::AddResponse {
            sum: request.get_ref().a + request.get_ref().b,
        }))
    }
}

async fn log(msg: &str) -> Error {
    Ok(tokio::io::stdout()
        .write_all(format!("{}\n", msg).as_bytes())
        .await?)
}

async fn server() -> Error {
    log("starting server").await?;

    tonic::transport::Server::builder()
        .add_service(service::service_server::ServiceServer::new(Service {}))
        .serve("127.0.0.1:5000".parse()?)
        .await?;
    Ok(())
}

async fn client() -> Error {
    log("starting client").await?;

    tokio::time::sleep(Duration::from_secs(3)).await;

    let client = service::service_client::ServiceClient::connect("http://127.0.0.1:5000").await;

    if let Err(_) = &client {
        log("client error").await?;
    }

    let mut client = client.unwrap();
    let res = client.add(service::AddRequest { a: 2, b: 2 }).await;

    match res {
        Ok(res) => {
            log(&format!("sum 2 + 2 = {}", res.get_ref().sum)).await?;
        }
        Err(_) => {
            log("client response error").await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Error {
    let (_, _) = tokio::join!(server(), client());
    Ok(())
}
