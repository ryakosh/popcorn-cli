pub fn casts_to_values(movie_id: i32, casts: &str) -> String {
    let mut values: String = casts
        .split(',')
        .map(|cast_id| format!("({},{}),", movie_id, cast_id))
        .collect();
    values.pop();

    values
}