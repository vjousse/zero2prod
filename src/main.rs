use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run("127.0.0.1:0")?.await
}
