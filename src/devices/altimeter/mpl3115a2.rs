struct MPL3115A2 {

}

impl MPL3115A2 {
    pub fn new() -> MPL3115A2 {
        unimplemented!()
    }
}

impl Altimeter for MPL3115A2 {

    fn get_altitude(&self) -> Meters {
        unimplemented!()
    }
}

impl Device for MPL3115A2 {
    fn get_name(&self) -> &str {
        "MPL3115A2"
    }
}