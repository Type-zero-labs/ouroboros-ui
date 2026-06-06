//! Guard test for the invariant premise: **atoms paint only with tokens**.
//!
//! Scans `src/atoms/**.rs` and fails if it finds a hardcoded design value — a `Color32`
//! built from literals or a named color constant, or a directly-constructed `FontId`.
//! Atoms must source every color from `Theme`/`core` and every font from
//! `theme::typography`. Sizes/radii/spacing come from `core::*` and are checked by review.

use std::fs;
use std::path::Path;

#[test]
fn atoms_use_only_tokens() {
    let mut violations = Vec::new();
    scan_dir(Path::new("src/atoms"), &mut violations);
    assert!(
        violations.is_empty(),
        "\nHardcoded design values found in atoms (use foundation tokens instead):\n  {}\n",
        violations.join("\n  ")
    );
}

fn scan_dir(dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            scan_file(&path, out);
        }
    }
}

fn scan_file(path: &Path, out: &mut Vec<String>) {
    let Ok(src) = fs::read_to_string(path) else {
        return;
    };
    for (i, line) in src.lines().enumerate() {
        // Strip line/doc comments so prose mentioning these patterns is not a violation.
        let code = line.split("//").next().unwrap_or("");
        let at = |what: &str| format!("{}:{}  {what}", path.display(), i + 1);

        if code.contains("Color32::from_") {
            out.push(at("Color32::from_* — use a Theme/core color token"));
        }
        if let Some(rest) = code.split("Color32::").nth(1) {
            // `Color32::WHITE`/`BLACK`/… (named const) — `Color32` as a bare type is fine.
            if rest.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
                out.push(at("Color32::<CONST> — use a Theme/core color token"));
            }
        }
        if code.contains("FontId::new(") {
            out.push(at(
                "FontId::new( — go through theme::typography (TypeStyle)",
            ));
        }
    }
}
