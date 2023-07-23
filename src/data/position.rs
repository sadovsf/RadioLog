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