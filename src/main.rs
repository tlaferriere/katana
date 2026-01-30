use clap::Parser;
use ninja_build_syntax::Statement;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    targets: Vec<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    // #[arg(short_alias = "C")]
    // #[command(subcommand)]
    // command: Option<Commands>,
}

fn main() {
    let build_ninja_path = "build.ninja";
    let buf = std::fs::read(Path::new(build_ninja_path))
        .expect(&format!("{build_ninja_path} not found."));
    let parsed = ninja_build_syntax::parse(&buf);
    let mut rules = HashMap::new();
    let mut builds = Vec::new();
    let mut bindings = Vec::new();
    let mut default = None;
    for (i, s) in parsed.enumerate() {
        // dbg!(&s);
        let Ok(stmt) = s.map_err(|err| {
            println!("Failed parsing line {i}:\n{err}");
        }) else {
            continue;
        };
        match stmt {
            Statement::Rule(rule) => {
                rules.insert(rule.name.0, rule);
            }
            Statement::Build(build) => {
                builds.push(build);
            }
            Statement::Binding(bind) => {
                bindings.push(bind);
            }
            Statement::Default(d) => {
                if default.is_some() {
                    println!("More than one default rule found at line {i}.")
                }
                default = Some(d);
            }
            Statement::Include(inc) => {
                todo!("Include another ninja file.");
            }
            Statement::Pool(pool) => {
                todo!("Handle pools.")
            }
            Statement::Comment(comment) => {
                // Ignore comments for now.
            }
        };
    }
}
