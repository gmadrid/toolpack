pub fn first<T, U>(tupl: &(T, U)) -> &T {
    &tupl.0
}

pub fn second<T, U>(tupl: &(T, U)) -> &U {
    &tupl.1
}

pub fn unwrap_second_or<'a, T, U>(tupl: &'a Option<(T, U)>, or_value: &'a U) -> &'a U {
    tupl.as_ref().map(second).unwrap_or(or_value)
}
