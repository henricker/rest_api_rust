use actix_web::{get, delete, web, Responder, HttpResponse, post, put};

use crate::{AppState, user_service::User};

#[get("/users")]
async fn get_all_users(app_data: web::Data<crate::AppState>) -> impl Responder {
    let action = app_data.service_manager.user.get().await.expect("Error in get users");
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users/{email}")]
async fn get_one_email(
    app_data: web::Data<crate::AppState>,
    email: web::Path<String>,
) -> impl Responder {
    let action = app_data.service_manager.user.get_user_email(&email).await.expect("Error in get user");
    let result = web::block(move || action).await.expect("Error to get user");
    HttpResponse::Ok().json(result)
}

 #[delete("/users/{email}")]
 async fn delete_user(app_data: web::Data<AppState>, email: web::Path<String>) -> impl Responder {
    let action = app_data.service_manager.user.delete(&email).await;
    let _result = web::block(move || action).await;
    HttpResponse::Ok()
 }

#[post("/users")]
async fn add_user(app_data: web::Data<crate::AppState>, user: web::Json<User>) -> impl Responder {
    print!("{}", user.first_name);
    let action = app_data.service_manager.user.create(&user).await.expect("Error during create user");
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.inserted_id),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[put("/users")]
async fn update_user(
    app_data: web::Data<crate::AppState>,
    user: web::Json<User>,
) -> impl Responder {
    let action = app_data.service_manager.user.update(&user).await.expect("Error in update user");
    let result = web::block(move || action).await;
    match result {
        Ok(result) => HttpResponse::Ok().json(result.modified_count),
        Err(e) => {
            println!("Error while getting, {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(get_one_email);
    cfg.service(add_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}