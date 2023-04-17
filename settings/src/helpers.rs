pub fn parse_usize(val: &str) -> Result<usize, String> {
    match val.parse::<usize>() {
        Ok(size) => Ok(size),
        Err(_) => Err(String::from("Invalid usize parameter")),
    }
}
