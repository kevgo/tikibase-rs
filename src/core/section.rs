use super::line::Line;

pub struct Section {
    /// The line number at which this section starts, 0-based.
    pub line_number: u32,
    /// Complete textual content of this section's title line, e.g. "# Title"
    pub title_line: String,
    /// Optional content of this section
    pub body: Vec<Line>,
}

impl Section {
    pub fn section_type(&self) -> String {
        let pos = self
            .title_line
            .char_indices()
            .find(|(_, letter)| *letter != '#' && *letter != ' ');
        match pos {
            None => "".to_string(),
            Some((pos, _)) => self.title_line.clone().split_off(pos),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_type() {
        let tests = vec![
            ("# Title", "Title"),
            ("### Title", "Title"),
            ("Title", "Title"),
            ("###", ""),
        ];
        for (give, want) in tests.into_iter() {
            let section = Section {
                line_number: 2,
                title_line: give.to_string(),
                body: vec![],
            };
            let have = section.section_type();
            assert_eq!(have, want.to_string(), "want: '{}', have: '{}'", want, have);
        }
    }
}
