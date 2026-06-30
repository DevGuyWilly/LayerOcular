use crate::configuration::{Settings};
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::{web::Data, dev::Server};
use secrecy::SecretString;
use crate::routes::{home};

pub struct Application {
    port : u16,
    server : Server,
}

pub struct ApplicationBaseUrl(pub String);

impl Application {
    pub async fn build(configuration : Settings) -> Result<Self, anyhow::Error> {

        let address = format!("{}:{}",
                              configuration.application.host,
                              configuration.application.port);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();

        let server = run(
            listener,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        ).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(
    listener : TcpListener,
    base_url : String,
    hmac_secret: SecretString,
) -> Result<Server, anyhow::Error> {

    let base_url = Data::new(ApplicationBaseUrl(base_url));
    let _hmac_secret = hmac_secret;

    // TODO:: Define my routes, for now '/' endpoint
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(home))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
