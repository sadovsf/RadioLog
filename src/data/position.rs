pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}


impl Position {
    pub fn new(latitude: f64, longitude: f64) -> Position {
        Position {
            latitude,
            longitude,
        }
    }
}