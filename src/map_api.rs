use std::{num::ParseFloatError, fmt::{Debug, Formatter}};

use serde::Deserialize;


pub struct OnlineMapError {
    message: String,
}

impl Debug for OnlineMapError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "OnlineMapError: {}", self.message)
    }
}

impl From<ureq::Error> for OnlineMapError {
    fn from(err: ureq::Error) -> Self {
        OnlineMapError {
            message: err.to_string(),
        }
    }
}

impl From<serde_json::Error> for OnlineMapError {
    fn from(err: serde_json::Error) -> Self {
        OnlineMapError {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for OnlineMapError {
    fn from(err: std::io::Error) -> Self {
        OnlineMapError {
            message: err.to_string(),
        }
    }
}

impl From<ParseFloatError> for OnlineMapError {
    fn from(err: ParseFloatError) -> Self {
        OnlineMapError {
            message: err.to_string(),
        }
    }
}



pub struct OnlineMap {

}

#[derive(Debug)]
pub struct LocationResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);


#[derive(Deserialize, Debug)]
struct OnlineLocation {
    // place_id: u64,
    // osm_type: String,
    // osm_id: u64,
    lat: String,
    lon: String,
    display_name: String,
    importance: f32,
}

impl OnlineMap {
    pub fn query_location(name :&String) -> Result<Vec<LocationResult>, OnlineMapError> {
        let resp = ureq::get(format!("https://nominatim.openstreetmap.org/search.php?q={}&format=json", name).as_str())
            .set("User-Agent", APP_USER_AGENT)
            .call()?
            .into_string()?;

        let mut list :Vec<OnlineLocation> = serde_json::from_str(&resp)?;
        list.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());

        let mut results = vec!();
        for loc in list {
            results.push(LocationResult {
                name: loc.display_name,
                latitude: loc.lat.parse::<f64>()?,
                longitude: loc.lon.parse::<f64>()?,
            });
        }
        Ok(results)
    }
}