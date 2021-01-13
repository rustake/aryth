#[cfg(test)]
mod tests {
    use aryth::Bound;

    #[test]
    fn f32_default_test() {
        let bound = Bound::<f32>::default();
        println!("{}, dif = {}", bound, bound.dif());
    }
}