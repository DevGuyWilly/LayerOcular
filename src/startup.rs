use crate::configuration::{Settings, DatabaseSettings};
use std::net::TcpListener;
use actix_web::{web, App, HttpServer};
use actix_web::{web::Data, dev::Server};
use actix_web::cookie::Key;
use secrecy::{ExposeSecret, SecretString};
use tracing_actix_web::TracingLogger;
use crate::routes::{home};
use crate::routes::{transactions, bank_connect, authentication, chat};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct Application {
    port : u16,
    server : Server,
}

pub struct ApplicationBaseUrl(pub String);

// Have a quick look over here again
#[derive(Clone)]
pub struct HmacSecret(pub SecretString);

impl Application {
    pub async fn build(configuration : Settings) -> Result<Self, anyhow::Error> {

        // Connection Pool
        let connection_pool = get_connection_pool(&configuration.database);

        let address = format!("{}:{}",
                              configuration.application.host,
                              configuration.application.port);

        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr()?.port();

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        ).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn run(
    listener : TcpListener,
    db_pool: PgPool,
    base_url : String,
    hmac_secret: SecretString,
) -> Result<Server, anyhow::Error> {

    let db_pool = Data::new(db_pool);

    let base_url = Data::new(ApplicationBaseUrl(base_url));

    // Have a quick look over here again
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());

    // TODO:: Define my routes, for now '/' endpoint
    let server = HttpServer::new(move || {
        App::new()
            // Auto Instrument every request
            .wrap(TracingLogger::default())

            // Root Route, homepage
            .route("/", web::get().to(home))

            // API Versioned Route
            .service(web::scope("/api/v1")
                .configure(authentication::init)
                .configure(transactions::init)
                .configure(bank_connect::init)
                .configure(chat::init)
            )
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            // Have a quick look over here again
            .app_data(Data::new(HmacSecret(hmac_secret.clone())))
    })
        .listen(listener)?
        .run();

    Ok(server)
}
