fn greeting() -> String {
    String::from("Hello from silver-wombat!")
}

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greets() {
        assert!(greeting().contains("silver-wombat"));
    }
}
