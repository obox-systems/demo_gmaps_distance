use clap::Parser;
use dotenv::dotenv;
use gmaps_distance::utils::*;
use google_maps::prelude::*;
use secrecy::ExposeSecret;
use secrecy::Secret;

/// Google Maps Distance and Time Calculator
///
/// This program calculates the distance and time between hotels and POIs
///
/// The input file should be a JSON file containing an array of hotels and an array of POIs
#[derive(Parser)]
#[clap(version = "0.1.0", author = "Viktor Dudnik")]
struct Args {
  /// The path to the JSON file containing the hotels and POIs
  #[clap(short, long, default_value = "sample.json")]
  pub file: String,

  /// The path to the JSON file to write the distance and time to
  ///
  /// Note: If the file already exists, it will be overwritten
  #[clap(short, long, default_value = "distime.json")]
  pub output: String,

  /// The Google Maps API key
  ///
  /// Note: If not provided, the program will look for the GOOGLE_API_KEY environment variable
  ///
  /// If the environment variable is not set, the program will exit with an error
  #[clap(short, long, env = "GOOGLE_API_KEY")]
  pub api_key: Option<Secret<String>>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  dotenv().ok();
  let args = Args::parse();
  let api_key = if let Some(key) = args.api_key {
    key
  } else {
    eprintln!("No Google api key provided!");
    std::process::exit(1);
  };

  let google_maps_client = GoogleMapsClient::new(api_key.expose_secret());

  println!("Reading from file: {}", args.file);
  let data = read_from_file(&args.file)?;

  let distimes = data.get_distance_and_time(google_maps_client).await?;

  for distime in distimes.iter() {
    println!(
      "From: {}\nTo: {}\nDistance: {}m\nDuration: {}",
      distime.from, distime.to, distime.distance, distime.duration
    )
  }

  println!("Writing to file: {}", args.output);
  save_to_file(&distimes, &args.output)?;

  Ok(())
}
