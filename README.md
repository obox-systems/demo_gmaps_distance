# Google maps distance calculator

This app calculates the distances to nearby points of interest like grocery shops, bars, etc.

## Try it out!
1. Install [Rust](https://rustup.rs/)
2. Get your Google API Key
- Put your API Key inside the `.env` file as such:
```
GOOGLE_API_KEY="your key"
```
- Or simply put it in your env vars:
```bash
$ export GOOGLE_API_KEY="your key"
```
3. Run the app:
```bash
$ cargo run --release
```
It will output the distances from points of interest from the `sample.json` file.


# Task description 
Location Map API expert (Google or others) to calculate distances

We are looking for a person that has experience in distance calculation from location maps, Google, or others.

We will deliver geolocation pins from the hotels and we would need to calculate the distances to nearby points of interest like grocery shops, bars, restaurants, the center of the town, ski slopes, etc.

The script should run automatically or with some of our inputs of the POI pins. It is the subject of debate with the candidate in the actual possibilities ofthe particular location map that will be used.
