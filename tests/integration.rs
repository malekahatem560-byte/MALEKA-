#[cfg(test)]
mod tests {
    use crate::core::identity::IdentityKernel;

    #[test]
    fn test_kernel_init() {
        let kernel = IdentityKernel::new("HASH".into(), "KEY".into());
        assert!(kernel.verify_sovereignty());
    }
}
