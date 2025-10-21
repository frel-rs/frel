use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Determine workspace root (utils is a direct child of the workspace root)
    let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_dir
        .parent()
        .expect("utils should be a direct child of the workspace root")
        .to_path_buf();

    // Default doc directory and output path
    let doc_dir = workspace_root.join("./doc");
    let mut out_path = workspace_root.join("./target/compiled.md");

    // Allow optional CLI arg to override output path: `utils --out path` or `utils path`
    let mut args = env::args().skip(1);
    if let Some(arg1) = args.next() {
        if arg1 == "--out" || arg1 == "-o" {
            if let Some(p) = args.next() {
                out_path = PathBuf::from(p);
            } else {
                eprintln!("--out requires a path");
                std::process::exit(2);
            }
        } else {
            out_path = PathBuf::from(arg1);
        }
    }

    if !doc_dir.exists() {
        eprintln!("doc directory not found at {}", doc_dir.display());
        std::process::exit(1);
    }

    let mut files = Vec::new();
    collect_markdown(&doc_dir, &mut files)?;

    // If output lives inside doc/, skip it from inputs to avoid self-inclusion
    let out_path_abs = if out_path.is_absolute() {
        out_path.clone()
    } else {
        workspace_root.join(&out_path)
    };

    files.retain(|p| !paths_equal(p, &out_path_abs));

    // Sort for deterministic output
    files.sort_by(|a, b| a.to_string_lossy().to_lowercase().cmp(&b.to_string_lossy().to_lowercase()));

    // Ensure parent exists
    if let Some(parent) = out_path_abs.parent() { fs::create_dir_all(parent)?; }

    let mut out = File::create(&out_path_abs)?;

    for (i, file) in files.iter().enumerate() {
        if i > 0 {
            writeln!(out, "\n\n---\n")?;
        }

        let rel = file.strip_prefix(&doc_dir).unwrap_or(file);
        writeln!(out, "# File: {}\n", rel.display())?;

        let mut content = String::new();
        File::open(file)?.read_to_string(&mut content)?;
        out.write_all(content.as_bytes())?;
        if !content.ends_with('\n') { writeln!(out)?; }
    }

    eprintln!("Wrote {} file(s) into {}", files.len(), out_path_abs.display());
    Ok(())
}

fn collect_markdown(dir: &Path, acc: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_markdown(&path, acc)?;
        } else if is_markdown(&path) {
            acc.push(path);
        }
    }
    Ok(())
}

fn is_markdown(path: &Path) -> bool {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => matches!(ext.to_ascii_lowercase().as_str(), "md" | "markdown"),
        None => false,
    }
}

fn paths_equal(a: &Path, b: &Path) -> bool {
    // Best-effort equality without canonicalization (which may fail on non-existent paths)
    normalize(a) == normalize(b)
}

fn normalize(p: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for comp in p.components() {
        use std::path::Component;
        match comp {
            Component::CurDir => {}
            Component::ParentDir => { out.pop(); }
            c => out.push(c.as_os_str()),
        }
    }
    out
}

