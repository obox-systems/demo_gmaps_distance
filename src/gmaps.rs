use google_maps::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct HotelsAndPOIS {
  hotels: Vec<Waypoint>,
  pois: Vec<Waypoint>,
}

#[derive(Serialize, Debug)]
pub struct DistanceTime {
  pub from: String,
  pub to: String,
  pub distance: String,
  pub duration: String,
}

impl HotelsAndPOIS {
  pub async fn get_distance_and_time(
    self,
    client: GoogleMapsClient,
  ) -> anyhow::Result<Vec<DistanceTime>> {
    let distance_matrix = client
      .distance_matrix(self.hotels, self.pois)
      .execute()
      .await?;

    let mut distime = vec![];
    for (i, row) in distance_matrix.rows.into_iter().enumerate() {
      for (j, element) in row.elements.into_iter().enumerate() {
        distime.push(DistanceTime {
          from: distance_matrix.origin_addresses[i].clone(),
          to: distance_matrix.destination_addresses[j].clone(),
          distance: element.distance.map_or("unknown".into(), |d| d.text),
          duration: element.duration.map_or("unknown".into(), |d| d.text)
        });
      }
    }

    Ok(distime)
  }
}