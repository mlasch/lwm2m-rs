#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;
use coap_lite::CoapOption;
use coap_lite::option_value;
use crate::lwm2m_requests::registration_request::Lwm2mRegistrationRequest;
use coap_server::app::{CoapError, Request, Response};
use coap_server::{app, CoapServer, FatalServerError, UdpTransport};
mod device;
mod lwm2m_requests;

#[tokio::main]
async fn main() -> Result<(), FatalServerError> {
    println!("Starting lwm2mserver");
    let server = CoapServer::bind(UdpTransport::new("[::]:20017")).await?;
    server
        .serve(
            app::new()
                .resource(app::resource("/bs").post(handle_bootstrap_device))
                .resource(app::resource("/rd").post(handle_register_device))
                .resource(app::resource("/dp").post(handle_send))
        )
        .await
}

async fn handle_bootstrap_device(request: Request<SocketAddr>) -> Result<Response, CoapError> {

    let registration_request = Lwm2mRegistrationRequest::new(request.clone())?;
    let response = request.new_response();
    Ok(response)
}

async fn handle_register_device(request: Request<SocketAddr>) -> Result<Response, CoapError> {
    println!("Got a request: {:?}", request);
    let registration_request = Lwm2mRegistrationRequest::new(request.clone())?;
    println!("Got registration{:?}", registration_request);
    let mut response = request.new_response();
    let client_id = 0;
    let location_path: option_value::OptionValueString = format!("rd/{client_id}").as_bytes().to_vec().try_into().unwrap();
    response.message.add_option_as(CoapOption::LocationPath, location_path);
    Ok(response)
}

async fn handle_send(request: Request<SocketAddr>) -> Result<Response, CoapError> {
    let whom = request
        .unmatched_path
        .first()
        .cloned()
        .unwrap_or_else(|| "world".to_string());

    let mut response = request.new_response();
    response.message.payload = format!("Hello, {whom}").into_bytes();
    Ok(response)
}
