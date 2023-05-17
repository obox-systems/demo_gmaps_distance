use google_maps::prelude::*;
use serde::{Deserialize, Serialize};

/// Input object to transform to distances.
#[derive(Debug, Deserialize)]
pub struct HotelsAndPOIS {
  hotels: Vec<Waypoint>,
  pois: Vec<Waypoint>,
}

/// Output object containing distance and time for two locations.
#[derive(Debug, Serialize)]
pub struct DistanceTime {
  /// Starting location.
  pub from: String,
  /// Target location.
  pub to: String,
  /// Total distance.
  pub distance: String,
  /// Total duration.
  pub duration: String,
}

impl HotelsAndPOIS {
  /// Distance and time calculation.
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