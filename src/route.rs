use actix_web::{get, post, HttpResponse, Responder, web};
use actix_web::http::header::ContentType;
use qrcodegen::{
    QrCode,
    QrCodeEcc
};
use serde_derive::Deserialize;

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("welcome to api")
}

#[post("/echo")]
pub async fn echo_post(request_body: String) -> impl Responder {
    HttpResponse::Ok().body(request_body)
}

#[get("/echo")]
pub async fn echo_get() -> impl Responder {
    HttpResponse::Ok().body("echo path")
}

#[post("/qr")]
pub async fn qr_generator(request_body: String) -> impl Responder {
    let qr = QrCode::encode_text(&request_body, QrCodeEcc::Medium).unwrap();
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let border: i32 = 5;
    let dim = qr.size().checked_add(5_i32.checked_mul(2).unwrap()).unwrap();
    result += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dim);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0 .. qr.size() {
        for x in 0 .. qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    HttpResponse::Ok()
        .content_type("image/svg")
        .body(result)
}

#[derive(Deserialize)]
pub struct QrRequest {
    data: String
}

/// get qr with query string param
/// example: http://xxx/qr?data=hello-world
#[get("/qr")]
pub async fn qr_generator_get(web::Query(param): web::Query<QrRequest>) -> impl Responder {
    let data = param.data;
    let qr = QrCode::encode_text(&data, QrCodeEcc::High).unwrap();
    let mut result = String::new();
    result += "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n";
    result += "<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">\n";
    let border: i32 = 5;
    let dim = qr.size().checked_add(5_i32.checked_mul(2).unwrap()).unwrap();
    result += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dim);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0 .. qr.size() {
        for x in 0 .. qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    let mut html = String::from("<div style='height: 400;width: 400'>");
    html.push_str(&result);
    html.push_str("</div>");
    HttpResponse::Ok()
        .set(ContentType::html())
        .body(html)
}