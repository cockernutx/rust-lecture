use std::process::Command;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./tailwind.css",
            "-o",
            "./target/tailwind.css",
        ])
        .output()
        .expect("failed to execute process");
    Ok(())
}
