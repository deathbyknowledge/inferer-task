
use std::{net::ToSocketAddrs, sync::Arc};
use warp::Filter;
use serde::{Serialize, Deserialize};

use crate::{model::Model, encoding};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct State {
    // Min: -4.8 Max: 4.8
    cart_position: f32,
    // Min: -Inf Max: Inf
    cart_velocity: f32,
    // Min: ~ -0.418 Max: ~ 0.418
    pole_angle: f32,
    // Min: -Inf Max: Inf
    pole_angular_velocity: f32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Response {
    action: u8,
}

pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = ::std::env::args().collect();
    if args.len() != 3 {
        println!("usage: {} server ADDRESS:PORT", args[0]);
        return Ok(());
    }

    let addr = args[2]
        .to_socket_addrs()
        .unwrap()
        .next()
        .expect("could not parse address");
    
    // Before starting the server, intialise the model
    let model_file = encoding::ModelFile::from("model.o");
    // Use Arc to share the model across threads
    let model = Arc::new(Model::from_file(model_file));

    let store_filter = warp::any().map(move || model.clone());
    let predict = warp::path("predict")
        .and(warp::post())
        .and(warp::body::json())
        .and(store_filter)
        .map(move |state: State, model: Arc<Model>| {
            let action = model.predict([
                state.cart_position,
                state.cart_velocity,
                state.pole_angle,
                state.pole_angular_velocity]
            );
            let res = Response { action };
            Ok(warp::reply::json(&res))
        });
    
    warp::serve(predict).run(addr).await;
    Ok(())
}
