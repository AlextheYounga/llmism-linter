use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;
use llmism_linter::{MatchKind, lint, source_files};

#[derive(Debug, Parser)]
#[command(
    name = "llmisms",
    version,
    about = "Flag common LLMisms in Markdown and text files"
)]
struct Cli {
    /// Markdown or text file, or a directory to scan recursively
    path: PathBuf,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match run(cli) {
        Ok(true) => ExitCode::from(1),
        Ok(false) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("llmisms: {error}");
            ExitCode::from(2)
        }
    }
}

fn run(cli: Cli) -> Result<bool, Box<dyn std::error::Error>> {
    let mut found_any = false;

    for path in source_files(&cli.path)? {
        let source = fs::read_to_string(&path)?;

        for finding in lint(&source) {
            found_any = true;
            let message = match finding.match_kind {
                MatchKind::Exact => format!("`{}`", finding.phrase),
                MatchKind::Variation => format!("variation of `{}`", finding.phrase),
            };
            println!(
                "{}:{}:{}: warning: {} ({})",
                path.display(),
                finding.line,
                finding.column,
                finding.rule,
                message,
            );
        }
    }

    Ok(found_any)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn run_distinguishes_clean_files_from_files_with_findings() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("llmisms-cli-{unique}.md"));

        fs::write(&path, "Plain, direct prose.").expect("fixture should be written");
        assert!(!run(Cli { path: path.clone() }).expect("clean file should be linted"));

        fs::write(&path, "Let's unpack that.").expect("fixture should be updated");
        assert!(run(Cli { path: path.clone() }).expect("fixture should be linted"));

        fs::remove_file(path).expect("temporary file should be removed");
    }
}
