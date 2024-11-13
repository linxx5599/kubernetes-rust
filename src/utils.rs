pub fn validate_and_set_value<T: std::str::FromStr>(
    value: Option<&str>,
    default_value: &str,
) -> String
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut result = default_value.to_string();
    if let Some(val) = value {
        if let Ok(_) = val.parse::<T>() {
            result = val.to_string();
        }
    }
    result
}

#[derive(Debug)]
pub struct PaginationParams {
    // pub page: String,
    pub limit: String,
}