pub fn grep<'a>(
    query: &'a str,
    content: &'a str,
) -> impl Iterator<Item = &'a str> {
    content
        .lines()
        .filter(move |line| line.contains(query))
}

pub fn grep_case_insensitive<'a>(
    query: &'a str,
    content: &'a str,
) -> impl Iterator<Item = &'a str> {
    content
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query.to_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        let v: Vec<&str> = grep(query, content).collect();

        assert_eq!(vec!["safe, fast, productive."], v);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let v: Vec<&str> = grep_case_insensitive(query, content).collect();

        assert_eq!(vec!["Rust:", "Trust me."], v);
    }
}
