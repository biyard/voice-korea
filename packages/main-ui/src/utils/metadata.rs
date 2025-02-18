pub fn get_ext_from_name(name: &str) -> Option<String> {
    let ret: Vec<&str> = name.split(".").collect();
    if ret.len() >= 2 {
        let ext = ret.last().unwrap().to_string();
        if ext == "jpeg" {
            Some("jpg".to_string())
        } else {
            Some(ext)
        }
    } else {
        None
    }
}
