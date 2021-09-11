pub trait Altimeter {
    fn get_altitude(&self) -> Meters;
}