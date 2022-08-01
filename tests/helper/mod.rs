

pub fn check_option<T>(arg: &Option<T>) -> bool{
    match arg {
        Some(_) => true,
        None => false,
    }
}
