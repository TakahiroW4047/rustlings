// if3.rs
//
// Execute `rustlings hint if3` or use the `hint` watch subcommand for a hint.


#[derive(Debug, PartialEq)]
enum Locations {
    Beach = 1,
    Burrow = 2,
    Desert = 3,
    Unknown = 4,
}

pub fn animal_habitat(animal: &str) -> Locations {
    let identifier = if animal == "crab" {
        Locations::Beach
    } else if animal == "gopher" {
        Locations::Burrow
    } else if animal == "snake" {
        Locations::Desert
    } else {
        Locations::Unknown
    };

    identifier
}

// No test changes needed.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), Locations::Burrow)
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), Locations::Desert)
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), Locations::Beach)
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), Locations::Unknown)
    }
}
