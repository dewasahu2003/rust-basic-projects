// Web Microservice 
use actix_web::{get,web,App,HttpResponse,HttpServer,Responder};

#[get("/")]
async fn index()-> impl Responder{
    HttpResponse::Ok().body("calculator Microservice")  
}

#[get("/add/{num1}/{num2}")]
async fn add(info:web::Path<(i32,i32)>)-> impl Responder{
    let res = calculator::add(info.0,info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/sub/{num1}/{num2}")]
async fn sub(info:web::Path<(i32,i32)>)-> impl Responder{
    let res = calculator::sub(info.0,info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/mul/{num1}/{num2}")]
async fn mul(info:web::Path<(i32,i32)>)-> impl Responder{
    let res = calculator::mul(info.0,info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[get("/div/{num1}/{num2}")]
async fn div(info:web::Path<(i32,i32)>)-> impl Responder{
    let res = calculator::div(info.0,info.1);
    HttpResponse::Ok().body(res.to_string())
}

#[actix_web::main]
async fn main()-> std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(index).service(add).service(sub).service(mul).service(div)
    }

    ).bind(("127.0.0.1",8080))?.run().await
}