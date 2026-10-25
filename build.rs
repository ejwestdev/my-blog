fn main() {
    // include_dir! embeds these files at compile time, but cargo doesn't track
    // files read by macros. Re-run (and thus recompile) when content changes.
    println!("cargo:rerun-if-changed=content/blog");
    println!("cargo:rerun-if-changed=content/projects");
}
