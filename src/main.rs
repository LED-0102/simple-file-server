use actix_web::{get, App, HttpResponse, HttpServer, Responder, http::header};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use std::path::PathBuf;

const ZIP_FILENAME: &str = "Lab_final.zip";

#[get("/")]
async fn download_zip() -> actix_web::Result<impl Responder> {
    let path: PathBuf = ZIP_FILENAME.into();

    match File::open(&path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            Ok(HttpResponse::Ok()
                .append_header((header::CONTENT_TYPE, "application/zip"))
                .append_header((
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", ZIP_FILENAME),
                ))
                .streaming(stream))
        }
        Err(_) => Ok(HttpResponse::NotFound().body("File not found")),
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Serving {} at http://localhost:8081/", ZIP_FILENAME);

    HttpServer::new(|| {
        App::new()
            .service(download_zip)
    })
        .bind("127.0.0.1:8081")?
        .run()
        .await
}
