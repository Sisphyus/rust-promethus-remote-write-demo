use snap::raw;

use super::state::AppState;
use actix_web::{
    post, web, HttpResponse,
};
use protobuf::Message;

use super::protos::remote::WriteRequest;
use log;

#[post("/v1/remote/write")]
async fn echo(data: web::Data<AppState>, body: web::Bytes) -> HttpResponse {
    let mut decoder = raw::Decoder::new();
    match decoder.decompress_vec(body.as_ref()) {
        Err(e) => {
            log::error!("failed to decode data, reason: {:?}", e);
            return HttpResponse::InternalServerError()
                .body(format!("failed to decode data, reason: {:?}", e));
        }
        Ok(v) => match <WriteRequest as Message>::parse_from_bytes(&v[..]) {
            Ok(wr) => {
                if wr.get_timeseries().len() > 0 {
                    if let Err(e) = data.channel.send(wr) {
                        log::error!("failed to send data to buffer, reason: {:?}", e)
                    }
                }
            }
            Err(e) => {
                log::error!("failed to unmarshal data, reason: {:?}", e);
                return HttpResponse::InternalServerError()
                    .body(format!("failed to unmarshal data, reason: {:?}", e));
            }
        },
    }
    HttpResponse::Ok().body("success!")
}
