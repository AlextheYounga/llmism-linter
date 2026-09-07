mod rules;

use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use rules::RULES;
use rust_stemmers::{Algorithm, Stemmer};

#[derive(Debug, PartialEq, Eq)]
pub enum MatchKind {
    Exact,
    Variation,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Finding {
    pub line: usize,
    pub column: usize,
    pub rule: &'static str,
    pub phrase: &'static str,
    pub match_kind: MatchKind,
}

pub fn lint(source: &str) -> Vec<Finding> {
    let mut findings = Vec::new();
    let stemmer = Stemmer::create(Algorithm::English);
    let rule_stems: Vec<_> = RULES
        .iter()
        .map(|rule| meaningful_stems(rule.phrase, &stemmer))
        .collect();

    for (line_index, line) in source.lines().enumerate() {
        let normalized = normalize(line);

        for rule in RULES {
            let phrase = normalize(rule.phrase);
            let mut search_start = 0;

            while let Some(offset) = normalized[search_start..].find(&phrase) {
                let match_start = search_start + offset;
                let match_end = match_start + phrase.len();

                if has_word_boundaries(&normalized, match_start, match_end) {
                    let column = normalized[..match_start].chars().count() + 1;
                    findings.push(Finding {
                        line: line_index + 1,
                        column,
                        rule: rule.name,
                        phrase: rule.phrase,
                        match_kind: MatchKind::Exact,
                    });
                }

                search_start = match_end;
            }
        }

        for (sentence_start, sentence) in sentences(&normalized) {
            let sentence_stems = meaningful_stems(sentence, &stemmer);
            let sentence_column = normalized[..sentence_start].chars().count() + 1;
            let sentence_end_column = sentence_column + sentence.chars().count();

            for (rule, required_stems) in RULES.iter().zip(&rule_stems) {
                if required_stems.len() < 2
                    || !required_stems
                        .iter()
                        .all(|required| sentence_stems.contains(required))
                    || findings.iter().any(|finding| {
                        finding.line == line_index + 1
                            && finding.rule == rule.name
                            && finding.column >= sentence_column
                            && finding.column < sentence_end_column
                    })
                {
                    continue;
                }

                let leading_whitespace = sentence
                    .chars()
                    .take_while(|character| character.is_whitespace())
                    .count();
                findings.push(Finding {
                    line: line_index + 1,
                    column: normalized[..sentence_start].chars().count() + leading_whitespace + 1,
                    rule: rule.name,
                    phrase: rule.phrase,
                    match_kind: MatchKind::Variation,
                });
            }
        }
    }

    findings.sort_by_key(|finding| (finding.line, finding.column));
    findings
}

pub fn source_files(path: &Path) -> io::Result<Vec<PathBuf>> {
    if path.is_file() {
        if is_supported(path) {
            return Ok(vec![path.to_path_buf()]);
        }

        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("{} is not a Markdown or text file", path.display()),
        ));
    }

    if !path.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} does not exist", path.display()),
        ));
    }

    let mut files = Vec::new();
    collect_source_files(path, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_source_files(directory: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;

        if file_type.is_dir() {
            collect_source_files(&path, files)?;
        } else if file_type.is_file() && is_supported(&path) {
            files.push(path);
        }
    }

    Ok(())
}

fn is_supported(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| {
            extension.eq_ignore_ascii_case("md") || extension.eq_ignore_ascii_case("txt")
        })
}

fn normalize(text: &str) -> String {
    text.chars()
        .flat_map(char::to_lowercase)
        .map(|character| match character {
            '\u{2018}' | '\u{2019}' => '\'',
            '\u{201c}' | '\u{201d}' => '"',
            _ => character,
        })
        .collect()
}

fn has_word_boundaries(text: &str, start: usize, end: usize) -> bool {
    let starts_at_boundary = text[..start]
        .chars()
        .next_back()
        .is_none_or(|character| !is_word_character(character));
    let ends_at_boundary = text[end..]
        .chars()
        .next()
        .is_none_or(|character| !is_word_character(character));

    starts_at_boundary && ends_at_boundary
}

fn is_word_character(character: char) -> bool {
    character.is_alphanumeric() || character == '_'
}

fn sentences(line: &str) -> impl Iterator<Item = (usize, &str)> {
    line.split_inclusive(['.', '!', '?'])
        .scan(0, |start, sentence| {
            let sentence_start = *start;
            *start += sentence.len();
            Some((sentence_start, sentence))
        })
        .filter(|(_, sentence)| !sentence.trim().is_empty())
}

fn meaningful_stems(text: &str, stemmer: &Stemmer) -> Vec<String> {
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_lowercase)
        .filter(|word| !is_stop_word(word))
        .map(|word| stemmer.stem(&word).into_owned())
        .fold(Vec::new(), |mut stems, stem| {
            if !stems.contains(&stem) {
                stems.push(stem);
            }
            stems
        })
}

fn is_stop_word(word: &str) -> bool {
    matches!(
        word,
        "a" | "an"
            | "and"
            | "are"
            | "aren"
            | "as"
            | "at"
            | "be"
            | "been"
            | "between"
            | "both"
            | "but"
            | "by"
            | "do"
            | "does"
            | "doesn"
            | "don"
            | "for"
            | "from"
            | "here"
            | "i"
            | "in"
            | "into"
            | "is"
            | "isn"
            | "it"
            | "just"
            | "let"
            | "ll"
            | "me"
            | "more"
            | "no"
            | "not"
            | "of"
            | "on"
            | "one"
            | "or"
            | "re"
            | "s"
            | "t"
            | "that"
            | "the"
            | "there"
            | "these"
            | "this"
            | "to"
            | "ve"
            | "we"
            | "what"
            | "where"
            | "who"
            | "with"
            | "would"
            | "wouldn"
            | "you"
            | "your"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn finds_phrases_case_insensitively_with_locations() {
        let findings = lint("An introduction.\nLET'S UNPACK THAT before continuing.");

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].line, 2);
        assert_eq!(findings[0].column, 1);
        assert_eq!(findings[0].rule, "unpacking ritual");
    }

    #[test]
    fn treats_curly_and_straight_apostrophes_as_equivalent() {
        let findings = lint("It’s important to note that this works.");

        assert!(
            findings
                .iter()
                .any(|finding| { finding.rule == "compulsory caveat" && finding.column == 1 })
        );
    }

    #[test]
    fn reports_multiple_phrases_in_source_order() {
        let findings = lint("Ultimately, let's unpack that.");

        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].phrase, "Ultimately");
        assert_eq!(findings[1].phrase, "Let's unpack that");
    }

    #[test]
    fn does_not_match_vocabulary_inside_a_larger_word() {
        assert!(lint("The system behaved robustly.").is_empty());
    }

    #[test]
    fn finds_inflected_phrase_words_in_the_same_sentence() {
        let findings = lint("Those distinctions still mattered to the result.");

        assert!(findings.iter().any(|finding| {
            finding.rule == "significance announcement"
                && finding.phrase == "That distinction matters"
                && finding.match_kind == MatchKind::Variation
        }));
    }

    #[test]
    fn does_not_combine_phrase_words_across_sentences() {
        let findings = lint("The distinction is clear. Nothing else matters.");

        assert!(
            !findings
                .iter()
                .any(|finding| finding.rule == "significance announcement")
        );
    }

    #[test]
    fn prefers_an_exact_match_over_a_variation() {
        let findings = lint("That distinction matters.");
        let matching_findings: Vec<_> = findings
            .iter()
            .filter(|finding| finding.rule == "significance announcement")
            .collect();

        assert_eq!(matching_findings.len(), 1);
        assert_eq!(matching_findings[0].match_kind, MatchKind::Exact);
    }

    #[test]
    fn recursively_collects_only_markdown_and_text_files() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("llmisms-{unique}"));
        let nested = root.join("nested");
        fs::create_dir_all(&nested).expect("temporary directory should be created");
        fs::write(root.join("notes.md"), "text").expect("fixture should be written");
        fs::write(nested.join("copy.TXT"), "text").expect("fixture should be written");
        fs::write(root.join("image.png"), "data").expect("fixture should be written");

        let files = source_files(&root).expect("directory should be readable");

        assert_eq!(files, vec![nested.join("copy.TXT"), root.join("notes.md")]);
        fs::remove_dir_all(root).expect("temporary directory should be removed");
    }
}
