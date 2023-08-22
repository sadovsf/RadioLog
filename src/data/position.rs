use crate::app_errors::AppError;

pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

// Calculations written based of https://www.giangrandi.org/electronics/radio/qthloccalc.shtml

impl Position {
    pub fn new(latitude: f64, longitude: f64) -> Position {
        Position {
            latitude,
            longitude,
        }
    }

    pub fn from_qth(qth: &str) -> Result<Position, AppError> {
        let qth = qth.to_uppercase();

        // Constants.
        const STR_CHR_UP :&str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const STR_NUM :&str = "0123456789";

        // Calculate lat/lon.
        // 2nd digit: 10deg latitude slot.
        let lat_letter = &qth[1..2];
        let lat_index = STR_CHR_UP.find( lat_letter ).ok_or( AppError::InvalidQTHLocator)?;
        let mut lat = (lat_index * 10) as f64;


        // 1st digit: 20deg longitude slot.
        let lon_letter = &qth[0..1];
        let lon_index = STR_CHR_UP.find( lon_letter ).ok_or( AppError::InvalidQTHLocator)?;
        let mut lon = (lon_index * 20) as f64;

        lat += (STR_NUM.find( &qth[3..4]).ok_or(AppError::InvalidQTHLocator)? * 1) as f64;  // 4th digit: 1deg latitude slot.
        lon += (STR_NUM.find( &qth[2..3]).ok_or(AppError::InvalidQTHLocator)? * 2) as f64;  // 3rd digit: 2deg longitude slot.

        if qth.len() == 6 {
            lat += (STR_CHR_UP.find(&qth[5..6]).ok_or(AppError::InvalidQTHLocator)? as f64) * 2.5 / 60.0;   // 6th digit: 2.5min latitude slot.
            lon += (STR_CHR_UP.find(&qth[4..5]).ok_or(AppError::InvalidQTHLocator)? as f64) * 5.0 / 60.0;   // 5th digit: 5min longitude slot.
        }

        if qth.len() == 4 {
            // Get coordinates of the center of the square.
            lat += 0.5 * 1.0;
            lon += 0.5 * 2.0;
        }
        else {
            lat += 0.5 * 2.5 / 60.0;
            lon += 0.5 * 5.0 / 60.0;
        }

        // Locator lat/lon origin shift.
        lat -= 90.0;
        lon -= 180.0;

        Ok(Position {
            latitude: lat,
            longitude: lon,
        })
    }

    pub fn to_qth(&self) -> String {
        if (self.latitude < -90.0 || self.latitude > 90.0) || (self.longitude < -180.0 || self.longitude > 180.0) {
            return String::from("INVALID");
        }

        // Normalize to positive values.
        let lat = self.latitude + 90.0;
        let lon = self.longitude + 180.0;

        // Constants.
        const STR_CHR_UP :&str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        const STR_CHR_LO :&str = "abcdefghijklmnopqrstuvwxyz";
        const STR_NUM :&str = "0123456789";

        // Calculate QTH locator.
        let mut qth = String::new();
        qth.push(STR_CHR_UP.as_bytes()[(lon / 20.0).floor() as usize] as char); // 1st digit: 20deg longitude slot.
        qth.push(STR_CHR_UP.as_bytes()[(lat / 10.0).floor() as usize] as char); // 2nd digit: 10deg latitude slot.
        qth.push(STR_NUM.as_bytes()[((lon % 20.0) / 2.0).floor() as usize] as char); // 3rd digit: 2deg longitude slot.
        qth.push(STR_NUM.as_bytes()[((lat % 10.0) / 1.0).floor() as usize] as char); // 4th digit: 1deg latitude slot.
        qth.push(STR_CHR_LO.as_bytes()[((lon % 2.0) * (60.0 / 5.0)).floor() as usize] as char); // 5th digit: 5min longitude slot.
        qth.push(STR_CHR_LO.as_bytes()[((lat % 1.0) * (60.0 / 2.5)).floor() as usize] as char); // 6th digit: 2.5min latitude slot.

        return qth;
    }

    /// Implementation of Haversine distance between two points.
    pub fn distance_to(&self, end: &Position) -> Distance {
        let haversine_fn = |theta: f64| (1.0 - theta.cos()) / 2.0;

        let phi1 = self.latitude.to_radians();
        let phi2 = end.latitude.to_radians();
        let lambda1 = self.longitude.to_radians();
        let lambda2 = end.longitude.to_radians();

        let hav_delta_phi = haversine_fn(phi2 - phi1);
        let hav_delta_lambda = phi1.cos() * phi2.cos() * haversine_fn(lambda2 - lambda1);
        let total_delta = hav_delta_phi + hav_delta_lambda;

        Distance::from_meters((2.0 * 6371e3 * total_delta.sqrt().asin() * 1000.0).round() / 1000.0)
    }

    pub fn azimuth_to(&self, end: &Position) -> f64 {
        let phi1 = self.latitude.to_radians();
        let phi2 = end.latitude.to_radians();
        let lambda1 = self.longitude.to_radians();
        let lambda2 = end.longitude.to_radians();

        let y = (lambda2 - lambda1).sin() * phi2.cos();
        let x = phi1.cos() * phi2.sin() - phi1.sin() * phi2.cos() * (lambda2 - lambda1).cos();

        let mut azimuth = y.atan2(x).to_degrees() + 360.0;
        if azimuth > 360.0 { azimuth -= 360.0; }

        azimuth
    }
}






pub struct Distance {
    meters: f64
}

impl Distance {
    pub fn from_meters(meters: f64) -> Distance {
        Distance {
            meters: meters,
        }
    }

    pub fn km(&self) -> f64 {
        self.meters / 1000.0
    }
}