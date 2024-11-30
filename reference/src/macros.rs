#[macro_export]
macro_rules! map {
    ( $( $key:expr => $value:expr ),* ) => {
        HashMap::from([$(
            ($key, $value),
        )*])
    };
}
