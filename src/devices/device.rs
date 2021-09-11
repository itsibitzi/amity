trait Device {
    fn get_id(&self) -> u8;
    fn get_name(&self) -> &str;
}