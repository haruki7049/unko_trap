use actix_web::{get,App,HttpResponse,HttpServer,Responder};

#[get("/")]
async fn entrance() -> impl Responder {
    HttpResponse::Ok().body("Hi! There. Do you try check my server?")//Greeting and navigate to Trap
}

#[get("/trap")]
async fn trap() -> impl Responder {
    HttpResponse::Ok().body("Trap!! You are stupid!! hahaha...")//trapped!!
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(entrance)
            .service(trap)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
