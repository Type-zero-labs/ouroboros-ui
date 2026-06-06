//! Composition guard: **molecules compose atoms, they never paint primitives.**
//!
//! Scans `src/molecules/**.rs` and fails if it finds a direct paint call. If a molecule needs
//! to paint, the missing piece must become an atom (atoms may paint; see `src/atoms/surface.rs`).

use std::fs;
use std::path::Path;

const FORBIDDEN: &[&str] = &[
    "ui.painter(",
    ".painter()",
    "rect_filled",
    "rect_stroke",
    "circle_filled",
    "circle_stroke",
    "layout_no_wrap",
    "layout_job",
    ".galley(",
    "Shape::line",
    "hline(",
    "vline(",
];

#[test]
fn molecules_compose_never_paint() {
    let mut violations = Vec::new();
    scan_dir(Path::new("src/molecules"), &mut violations);
    scan_dir(Path::new("src/cells"), &mut violations);
    assert!(
        violations.is_empty(),
        "\nMolecules must compose atoms, not paint primitives:\n  {}\n",
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
        let code = line.split("//").next().unwrap_or("");
        for pat in FORBIDDEN {
            if code.contains(pat) {
                out.push(format!(
                    "{}:{}  `{pat}` — compose an atom instead",
                    path.display(),
                    i + 1
                ));
            }
        }
    }
}
