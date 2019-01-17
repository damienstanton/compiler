#[cfg(test)]
mod tests {
    use compiler::simple_math::inc;
    
    #[test]
    fn smoke() {
        assert_eq!(inc(1), 2);
    }
}