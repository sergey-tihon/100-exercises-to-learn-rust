fn intro() -> &'static str {
    "I'm ready to refine the `Ticket` type!"
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to refine the `Ticket` type!");
    }
}
