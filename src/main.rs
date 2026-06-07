use std::env;
use google_maps::prelude::*;

const OFFICE_MILAGE: f32 = 2.5;

#[tokio::main]
async fn main() -> Result<(), String> {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args.len() > 4 {
        return Err(format!("Not enough args"))
    }

    let postcode_1: String = args[1].to_string();
    let postcode_2: String = args[2].to_string();

    println!("Calculating Distance from {} to {}",postcode_1, postcode_2);

    let distance = calc_distance(postcode_1, postcode_2).await;

    Ok(())
}

#[allow(dead_code)]
async fn calc_distance(postcode_1: String, postcode_2: String) -> Result<DirectionsResponse, Error> {
    let google_maps_client = google_maps::Client::try_new("AIzaSyAdkoIBlhkV8K8gdYWKhSFEa_on1Oi5BE8")?;

    let directions = google_maps_client.directions(
        Location::from_address(postcode_1),
        Location::from_address(postcode_2),
    )
    .with_travel_mode(TravelMode::Driving)
    .execute()
    .await?;

    println!("{:#?}", directions);

    Ok(directions)
    // Reponse { Routes [ legs [  steps [ Step { Distance }]]]} We need Step Distance
    // arrival_time: None,
    // departure_time: None,
    // distance: DirectionsDistance {
    //     text: "101 km",
    //     value: 100675,
    // },


}
