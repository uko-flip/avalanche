pub mod compiler;
pub mod config;
pub mod lexer;

#[cfg(test)]
mod tests {
    use rustc_version::version;

    #[test]
    fn test_compiler_version() {
        assert!(version().unwrap().major >= 1);
    }
}
