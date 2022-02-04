#[derive(Copy, Clone)]
pub struct CoordPoint {
    x: f64,
    y: f64,
}

impl CoordPoint {
    pub fn new(nx: f64, ny: f64) -> CoordPoint {
        CoordPoint { x: nx, y: ny }
    }
    pub fn set_location(&mut self, nx: f64, ny: f64) {
        self.x = nx;
        self.y = ny;
    }
    pub fn get_distance(self, nx: f64, ny: f64) -> f32 {
        ((nx - self.x).powi(2) + (ny - self.y).powi(2)).sqrt() as f32
    }
    pub fn get_x(self) -> f64 {
        self.x
    }
    pub fn get_y(self) -> f64 {
        self.y
    }
}
pub struct AnimalTracker {
    pub name: String,
    pub location: CoordPoint,
    distance_travelled: f64,
}
impl AnimalTracker {
    pub fn new(nme: String, nx: f64, ny: f64) -> AnimalTracker {
        AnimalTracker {
            name: nme,
            location: CoordPoint::new(nx, ny),
            distance_travelled: 0.0f64,
        }
    }
    pub fn get_distance_travelled(&mut self) -> f64 {
        self.distance_travelled
    }
    pub fn move_(&mut self, nx: f64, ny: f64) {
        self.distance_travelled += self.location.get_distance(nx, ny) as f64;
        self.location.set_location(nx, ny)
    }
}
