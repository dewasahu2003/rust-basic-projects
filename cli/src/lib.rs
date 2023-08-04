//! A Marco Polo Game

pub fn marco_polo(name: &str) -> String {
    if name == "marco" {
        "Polo".to_string()
    } else {
        "what's your name".to_string()
    }
}
