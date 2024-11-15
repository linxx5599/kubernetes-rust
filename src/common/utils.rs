pub fn validate_and_set_value<T: std::str::FromStr>(value: Option<&str>) -> String
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut result: String = "".to_string();
    if let Some(val) = value {
        if let Ok(_) = val.parse::<T>() {
            result = val.to_string();
        }
    }
    result
}

pub fn validate_and_set_str<T: std::str::FromStr>(value: Option<&str>) -> String
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut result: String = "".to_string();
    if let Some(val) = value {
        result = val.to_string();
    }
    result
}

#[derive(Debug)]
pub struct PaginationParams {
    // pub page: String,
    pub limit: String,
}


pub fn get_root_error(err: &dyn std::error::Error) -> &dyn std::error::Error {
    let mut current_err = err;
    while let Some(source) = current_err.source() {
        current_err = source;
    }
    current_err
}
