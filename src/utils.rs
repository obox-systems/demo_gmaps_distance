use crate::gmaps::HotelsAndPOIS;

/// Read locations from a JSON file.
pub fn read_from_file(path: &str) -> anyhow::Result<HotelsAndPOIS> {
  let file = std::fs::File::open(path)?;
  let buffer = std::io::BufReader::new(file);
  let data = serde_json::from_reader(buffer)?;
  Ok(data)
}

/// Save distance and time to file.
pub fn save_to_file<T: serde::Serialize>(data: &T, path: &str) -> anyhow::Result<()> {
  let file = std::fs::File::create(path)?;
  let buffer = std::io::BufWriter::new(file);
  serde_json::to_writer_pretty(buffer, data)?;
  Ok(())
}