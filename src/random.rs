pub trait Random<T, G> {
    fn random(quality: G) -> T;
}