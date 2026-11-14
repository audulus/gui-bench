//! The text font. Lorelei shapes text from a table it derives from the font, and at the pinned
//! revision it has that table only for fonts with little Latin substitution; with any other
//! font the window panics at its first text. So the app takes the first sans-serif face that
//! Lorelei can shape: the system `sans-serif` that Lorelei itself defaults to, then a few
//! common families.

use std::process::Command;

use bough_text::script::SimpleScript;

/// The families after the system `sans-serif`, in order.
const FAMILIES: [&str; 5] = ["Cantarell", "DejaVu Sans", "Arial", "Helvetica", "Verdana"];

/// Returns the bytes of the font of the app, or exits when the system has none that fits.
pub fn load() -> Vec<u8> {
    let system = find("sans-serif", false);
    let named = FAMILIES.iter().filter_map(|family| find(family, true));
    for path in system.into_iter().chain(named) {
        let Ok(data) = std::fs::read(&path) else {
            continue;
        };
        if shapes(&data) {
            eprintln!("lorelei-bench: font {path}");
            return data;
        }
        eprintln!("lorelei-bench: Lorelei cannot shape {path}");
    }
    eprintln!("lorelei-bench: no sans-serif font that Lorelei can shape");
    std::process::exit(1);
}

/// Returns true when Lorelei has its shaping table for a font, with the part its flow text
/// needs.
fn shapes(data: &[u8]) -> bool {
    SimpleScript::build_cached(data, 0).is_some_and(|script| script.tokens.is_some())
}

/// Returns the file that fontconfig matches to `family`. With `exact`, only when the match is
/// that family and no substitute.
fn find(family: &str, exact: bool) -> Option<String> {
    let output = Command::new("fc-match")
        .args(["-f", "%{family}\n%{file}", family])
        .output()
        .ok()
        .filter(|output| output.status.success())?;
    let text = String::from_utf8(output.stdout).ok()?;
    let (families, file) = text.split_once('\n')?;
    let matched = !exact
        || families
            .split(',')
            .any(|name| name.eq_ignore_ascii_case(family));
    (matched && !file.is_empty()).then(|| file.to_owned())
}
