use dotenv::dotenv;
use actix_web::{middleware, App, HttpServer, http, web::Data};
use std::env;
use mongodb::{options::ClientOptions, Client, Collection};
use actix_cors::Cors;
use user_service::{UserService, User};

mod user_service;
mod user_router;

pub struct ServiceManager {
    user: UserService,
}


impl ServiceManager {
    pub fn new(user: UserService) -> Self {
        ServiceManager { user }
    }
}

pub struct AppState {
    service_manager: ServiceManager,
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // init env
    dotenv().ok();

   let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set in .env file");
   let user_collection = get_user_collection().await;

   // init logger middleware
   env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
   env_logger::init();

   HttpServer::new( move || {
    let user_service_worker = UserService::new(user_collection.clone());
    let service_manager = ServiceManager::new(user_service_worker);
    
    // cors
    let cors_middleware = Cors::default()
        .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

    App::new()
        .wrap(middleware::Logger::default())
        .wrap(cors_middleware)
        .app_data(Data::new(AppState { service_manager }))
        .configure(user_router::init)
    })
    .bind(server_url)?
    .run()
    .await

}


async fn get_user_collection() -> Collection<User> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let client_options = ClientOptions::parse(&database_url).await.expect("Erro on client options");
    let client_mongo = Client::with_options(client_options).expect("Failed to create client");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME is not set in .env file");
    let db = client_mongo.database(&database_name);
    let user_collection_name = env::var("USER_COLLECTION_NAME").expect("USER_COLLECTION_NAME is not set in .env file");
    let user_collection: Collection<User> = db.collection(&user_collection_name);
    return user_collection
}