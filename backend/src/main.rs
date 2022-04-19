use actix_web::{HttpServer, App, web, Responder};
use serde_json::json;
use actix_web_lab::web::spa;

async fn get_hi()->impl Responder{
	web::Json(json!({"message":"hello"}))
}

#[actix_web::main]
async fn main()->std::io::Result<()> {

	HttpServer::new(||{
		App::new()
			.route("api/hi", web::get().to(get_hi))
			.service(
				spa()
					// ln -s ../frontend/dist ./dist
					.index_file("./dist/index.html")
					.static_resources_mount("/")
					.static_resources_location("./dist")
					.finish()
			)
	}).bind(("0.0.0.0",8090))?.run().await

}
