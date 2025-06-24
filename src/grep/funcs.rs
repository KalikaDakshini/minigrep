use super::config::Config;
use std::error::Error;
use std::fs;

/// Runs the grep command
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents of file to a string
    let contents = fs::read_to_string(config.path())?;

    // Match by case
    let results = if config.ignore_case() {
        search_insensitive(config.query(), &contents)
    } else {
        search(config.query(), &contents)
    };

    // Print the match results
    for (idx, line) in results {
        println!("{}: {line}", idx + 1);
    }
    Ok(())
}

/// Search the contents for query and return the lines containing query
fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query = query.to_lowercase();
    contents
        .lines()
        .enumerate()
        .filter(|(_, line)| line.to_lowercase().contains(&query))
        .collect()
}

// Tests for TDD
#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_search_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct Tape";
        assert_eq!(
            vec![(1, "safe, fast, productive.")],
            search(query, contents)
        );
    }

    #[test]
    fn test_search_insensitive() {
        let query = "rUst";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me\n";
        assert_eq!(
            vec![(0, "Rust:"), (3, "Trust me")],
            search_insensitive(query, contents)
        );
    }
}
