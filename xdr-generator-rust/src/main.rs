//! CLI entry point for the XDR to Rust code generator.

use askama::Template;
use clap::Parser;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use xdr_generator_rust::generator::Generator;
use xdr_generator_rust::parser;
use xdr_generator_rust::types::Options;

/// XDR to Rust code generator.
#[derive(Parser, Debug)]
#[command(name = "xdr-generator-rust")]
#[command(about = "Generate Rust code from XDR definitions")]
struct Args {
    /// Input XDR files
    #[arg(short, long, required = true)]
    input: Vec<PathBuf>,

    /// Output Rust file
    #[arg(short, long)]
    output: PathBuf,

    /// Types with custom Default implementation (skip derive(Default))
    #[arg(long, value_delimiter = ',')]
    custom_default: Vec<String>,

    /// Types with custom FromStr/Display implementation (use SerializeDisplay)
    #[arg(long, value_delimiter = ',')]
    custom_str: Vec<String>,
}

/// Extract include dependencies from XDR file content.
/// Returns a list of base names (e.g., "Stellar-types" from "xdr/Stellar-types.h").
fn extract_includes(content: &str) -> Vec<String> {
    let mut includes = Vec::new();
    for line in content.lines() {
        // Match both `%#include` and `% #include` patterns
        let trimmed = line.trim();
        if trimmed.starts_with("%#include") || trimmed.starts_with("% #include") {
            // Extract the path from quotes
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let path = &trimmed[start + 1..start + 1 + end];
                    // Extract the base name (e.g., "Stellar-types" from "xdr/Stellar-types.h")
                    let base = Path::new(path)
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");
                    if !base.is_empty() {
                        includes.push(base.to_string());
                    }
                }
            }
        }
    }
    includes
}

/// Get the base name from a file path (e.g., "Stellar-types" from "xdr/curr/Stellar-types.x").
fn get_base_name(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_string()
}

/// Topologically sort files based on include dependencies.
/// Returns files in order where dependencies come before dependents.
fn sort_by_dependencies(files: &[(PathBuf, String)]) -> Vec<(PathBuf, String)> {
    // Build a map from base name to (path, content)
    let mut file_map: HashMap<String, (PathBuf, String)> = HashMap::new();
    for (path, content) in files {
        let base = get_base_name(path);
        file_map.insert(base, (path.clone(), content.clone()));
    }

    // Build dependency graph: base_name -> list of dependencies (base names)
    let mut deps: HashMap<String, Vec<String>> = HashMap::new();
    for (path, content) in files {
        let base = get_base_name(path);
        let includes = extract_includes(content);
        deps.insert(base, includes);
    }

    // Topological sort using Kahn's algorithm
    // First, compute in-degrees
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for base in file_map.keys() {
        in_degree.insert(base.clone(), 0);
    }
    for dep_list in deps.values() {
        for dep in dep_list {
            if file_map.contains_key(dep) {
                *in_degree.entry(dep.clone()).or_insert(0) += 1;
            }
        }
    }

    // Actually, we want reverse: files with no dependencies come first
    // Recompute: in-degree counts how many files depend ON this file
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for base in file_map.keys() {
        in_degree.insert(base.clone(), 0);
    }
    for (base, dep_list) in &deps {
        for dep in dep_list {
            if file_map.contains_key(dep) {
                // `base` depends on `dep`, so dep must come before base
                // We increment in_degree of base (the dependent)
                *in_degree.entry(base.clone()).or_insert(0) += 1;
            }
        }
    }

    // Build reverse adjacency: dep -> list of files that depend on it
    let mut reverse_adj: HashMap<String, Vec<String>> = HashMap::new();
    for base in file_map.keys() {
        reverse_adj.insert(base.clone(), Vec::new());
    }
    for (base, dep_list) in &deps {
        for dep in dep_list {
            if file_map.contains_key(dep) {
                reverse_adj
                    .entry(dep.clone())
                    .or_default()
                    .push(base.clone());
            }
        }
    }

    // Find all nodes with in-degree 0 (no dependencies)
    let mut queue: Vec<String> = in_degree
        .iter()
        .filter(|(_, &deg)| deg == 0)
        .map(|(name, _)| name.clone())
        .collect();
    // Sort for deterministic order among files with same dependency level
    queue.sort();

    let mut result: Vec<String> = Vec::new();
    while let Some(current) = queue.pop() {
        result.push(current.clone());

        // For each file that depends on current, decrement its in-degree
        if let Some(dependents) = reverse_adj.get(&current) {
            let mut new_ready: Vec<String> = Vec::new();
            for dependent in dependents {
                if let Some(deg) = in_degree.get_mut(dependent) {
                    *deg -= 1;
                    if *deg == 0 {
                        new_ready.push(dependent.clone());
                    }
                }
            }
            // Sort new ready nodes and add to queue
            new_ready.sort();
            // We're using pop(), so reverse to maintain order
            new_ready.reverse();
            queue.extend(new_ready);
        }
    }

    // Reverse because we used pop (LIFO)
    // Actually, let's use a proper queue approach
    // Let me redo this more carefully

    // Restart with proper BFS
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    for base in file_map.keys() {
        in_degree.insert(base.clone(), 0);
    }
    for (base, dep_list) in &deps {
        for dep in dep_list {
            if file_map.contains_key(dep) {
                *in_degree.entry(base.clone()).or_insert(0) += 1;
            }
        }
    }

    let mut result: Vec<String> = Vec::new();
    let mut remaining: HashSet<String> = file_map.keys().cloned().collect();

    while !remaining.is_empty() {
        // Find all files with in-degree 0 among remaining
        let mut ready: Vec<String> = remaining
            .iter()
            .filter(|name| in_degree.get(*name).copied().unwrap_or(0) == 0)
            .cloned()
            .collect();

        if ready.is_empty() {
            // Circular dependency - just take remaining in sorted order
            let mut rem: Vec<_> = remaining.into_iter().collect();
            rem.sort();
            result.extend(rem);
            break;
        }

        // Sort for deterministic order
        ready.sort();

        for name in &ready {
            result.push(name.clone());
            remaining.remove(name);

            // Decrement in-degree of files that depend on this one
            for (dependent, dep_list) in &deps {
                if dep_list.contains(name) {
                    if let Some(deg) = in_degree.get_mut(dependent) {
                        if *deg > 0 {
                            *deg -= 1;
                        }
                    }
                }
            }
        }
    }

    // Convert base names back to files
    result
        .into_iter()
        .filter_map(|base| file_map.remove(&base))
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read all input files
    let mut files: Vec<(PathBuf, String)> = Vec::new();
    for path in &args.input {
        let content = fs::read_to_string(path)?;
        files.push((path.clone(), content));
    }

    // Sort files by include dependencies
    let sorted_files = sort_by_dependencies(&files);

    // Build combined XDR in dependency order
    let mut combined_xdr = String::new();
    for (_, content) in &sorted_files {
        combined_xdr.push_str(content);
        combined_xdr.push('\n');
    }

    // Build input_files list (sorted by path for SHA256 hashes)
    let mut input_files: Vec<(String, String)> = sorted_files
        .iter()
        .map(|(path, content)| (path.to_string_lossy().to_string(), content.clone()))
        .collect();
    input_files.sort_by(|a, b| a.0.cmp(&b.0));

    // Parse the combined XDR
    let spec = parser::parse(&combined_xdr)?;

    // Read the header file
    let header_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("xdr-generator")
        .join("generator")
        .join("header.rs");
    let header = fs::read_to_string(&header_path)?;

    // Build options
    let options = Options {
        custom_default_impl: args.custom_default.into_iter().collect::<HashSet<_>>(),
        custom_str_impl: args.custom_str.into_iter().collect::<HashSet<_>>(),
    };

    // Generate the output
    let generator = Generator::new(&spec, options);
    let template = generator.generate(&spec, &input_files, &header);

    // Render the template
    let output = template.render()?;

    // Write the output
    fs::write(&args.output, output)?;

    eprintln!("Generated: {}", args.output.display());

    Ok(())
}
