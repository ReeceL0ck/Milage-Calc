use std::env;
use google_maps::prelude::*;
// use serde::{Serialize, Deserialize};
// use serde_json::{Result, Value};

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

    let distance = calc_distance(postcode_1, postcode_2)
        .await
        .map_err(|e| e.to_string())?;

    let milage = distance - OFFICE_MILAGE;

    println!("Mileage after office deduction: {:.2}", milage);

    Ok(())
}

async fn calc_distance(postcode_1: String, postcode_2: String) -> Result<f32,  Box<dyn std::error::Error>> {
    let google_maps_client = google_maps::Client::try_new("AIzaSyAdkoIBlhkV8K8gdYWKhSFEa_on1Oi5BE8")?;

    let directions = google_maps_client.directions(
        Location::from_address(postcode_1),
        Location::from_address(postcode_2),
    )
    .with_travel_mode(TravelMode::Driving)
    .execute()
    .await?;

    // println!("{:#?}", directions);

    let (_text, distance) = directions
        .routes
        .first()
        .and_then(|route| route.legs.first())
        .map(|leg| (leg.distance.text.clone(), leg.distance.value))
        .ok_or("No route found")?;

    let km = distance as f32 / 1000.0;

    // println!("{}", km);

    let miles = convert_km_to_miles(km);

    // println!("Miles : {miles}");
    Ok(miles)

}


fn convert_km_to_miles(km:f32) -> f32 {
    return km /  1.609;
}