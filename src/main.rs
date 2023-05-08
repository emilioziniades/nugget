use clap::Parser;
use std::{collections::HashMap, io, process::Command};

/// command line tool for better nuget management
#[derive(Parser, Debug)]
struct Args {
    /// automatically update nugets, default is interactive
    #[clap(long, short, action)]
    auto: bool,
    /// space separated list of prefixes
    #[clap(long, short, num_args = 0..)]
    prefixes: Vec<String>,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dependency {
    name: String,
    requested: String,
    resolved: String,
    latest: String,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
    let output = Command::new("dotnet")
        .arg("list")
        .arg("package")
        .arg("--outdated")
        .output()
        .unwrap();
    let output = std::str::from_utf8(&output.stdout).unwrap();
    println!("{}", output);

    let mut dependency_map = HashMap::new();

    let mut current_project: String = String::new();

    for line in output.lines() {
        if line.starts_with("The given project") {
            current_project = text_io::read!(
                "The given project `{}` has no updates given the current sources.",
                line.bytes()
            );
        } else if line.starts_with("Project") {
            current_project = text_io::read!(
                "Project `{}` has the following updates to its packages",
                line.bytes()
            );
        } else if line.trim_start().starts_with(">") {
            let dependency_details = line.trim_start().replace('>', "");
            let dependency_details: Vec<&str> = dependency_details.split_whitespace().collect();
            let dependency = Dependency {
                name: dependency_details[0].to_string(),
                requested: dependency_details[1].to_string(),
                resolved: dependency_details[2].to_string(),
                latest: dependency_details[3].to_string(),
            };

            dependency_map
                .entry(dependency)
                .or_insert(vec![])
                .push(current_project.clone());
        }
    }

    for (dependency, projects_used) in dependency_map {
        println!(
            "update {} ({}): {:?}? (y, n, q)",
            dependency.name.clone(),
            dependency.latest,
            projects_used
        );

        loop {
            let mut decision = String::new();
            io::stdin().read_line(&mut decision).unwrap();
            match decision.as_str().trim() {
                "y" => {
                    println!("installing!");
                    for project_used in projects_used {
                        let output = Command::new("dotnet")
                            .arg("add")
                            .arg(project_used)
                            .arg("package")
                            .arg(dependency.name.clone())
                            .output()
                            .unwrap();
                        println!("{output:?}");
                        let output = std::str::from_utf8(&output.stdout).unwrap();
                        println!("{}", output);
                    }
                    break;
                }
                "n" => {
                    println!("not installing!");
                    break;
                }
                "q" => {
                    println!("quitting");
                    return;
                }
                _ => {
                    println!("unrecognized input");
                    continue;
                }
            }
        }
    }
}
