use clap::Parser;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::{exit, Command, Output},
};

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

trait OutputDisplayer: Sized {
    fn print_output(self) -> Self;
    fn get_output(self) -> String;
}

impl OutputDisplayer for Output {
    fn print_output(self) -> Self {
        println!("{}", stringify_bytes(&self.stdout));
        println!("{}", stringify_bytes(&self.stderr));
        self
    }

    fn get_output(self) -> String {
        stringify_bytes(&self.stdout).to_string()
    }
}

struct Dotnet;

impl Dotnet {
    fn restore() -> Output {
        Command::new("dotnet").arg("restore").output().unwrap()
    }

    fn list_outdated() -> Output {
        Command::new("dotnet")
            .arg("list")
            .arg("package")
            .arg("--outdated")
            .output()
            .unwrap()
    }

    fn update_package(project: &str, dependency: &str, version: &str) -> Output {
        Command::new("dotnet")
            .arg("add")
            .arg(project)
            .arg("package")
            .arg(dependency)
            .arg("--version")
            .arg(version)
            .arg("--no-restore") // skip restoring as dotnet will complain about package downgrades
            .output()
            .unwrap()
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Dependency {
    name: String,
    requested: String,
    resolved: String,
    latest: String,
}

impl From<&str> for Dependency {
    // assumes a dependency is expressed as a line of the following form:
    //              > ThePackage        1.0.11        1.0.11        1.0.14
    //                ^ name            ^ requested   ^resolved     ^ latest
    //  it is incredibly brittle and may break at any moment. Unfortunately
    //  dotnet does not provide the data in any structured form such as JSON.
    fn from(line: &str) -> Self {
        let dependency_details = line.trim_start().replace('>', "");
        let dependency_details: Vec<&str> = dependency_details.split_whitespace().collect();
        Self {
            name: dependency_details[0].to_string(),
            requested: dependency_details[1].to_string(),
            resolved: dependency_details[2].to_string(),
            latest: dependency_details[3].to_string(),
        }
    }
}

enum Action {
    Confirm,
    Deny,
    Quit,
    Other,
}

impl Action {
    fn from_stdin(prompt: &str) -> Self {
        print!("{} ", prompt);
        stdout().flush().unwrap();

        let action = {
            let mut action = String::new();
            stdin().read_line(&mut action).unwrap();
            action.trim().to_lowercase()
        };

        match action.as_str() {
            "y" | "yes" => Self::Confirm,
            "n" | "no" => Self::Deny,
            "q" | "quit" => Self::Quit,
            _ => Self::Other,
        }
    }
}

fn main() {
    let args = Args::parse();

    let outdated_dependencies = if !args.prefixes.is_empty() {
        get_outdated_dependencies()
            .into_iter()
            .filter(|(dependency, _)| {
                args.prefixes
                    .iter()
                    .any(|prefix| dependency.name.starts_with(prefix))
            })
            .collect()
    } else {
        get_outdated_dependencies()
    };

    for (dependency, projects) in outdated_dependencies {
        // automatic install
        if args.auto {
            update_dependency(dependency, &projects);
            continue;
        }

        //interactive install
        let prompt = format!(
            "update {} ({} -> {}): {:?}? (y, n, q)",
            &dependency.name, dependency.resolved, dependency.latest, projects
        );

        loop {
            match Action::from_stdin(&prompt) {
                Action::Confirm => {
                    println!("installing");
                    update_dependency(dependency, &projects);
                    break;
                }
                Action::Deny => {
                    println!("not installing");
                    break;
                }
                Action::Quit => {
                    println!("quitting");
                    return;
                }
                Action::Other => {
                    println!("unrecognized input, try again");
                    continue;
                }
            }
        }
    }

    // restore once all upgrades have happened
    Dotnet::restore().print_output();
}

fn get_outdated_dependencies() -> HashMap<Dependency, Vec<String>> {
    let output = Dotnet::list_outdated().print_output();

    if !output.status.success() {
        exit(output.status.code().unwrap())
    }

    let output = output.get_output();

    let mut dependency_map = HashMap::new();
    let mut current_project = String::new();

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
        } else if line.contains('>') {
            let dependency = Dependency::from(line);
            dependency_map
                .entry(dependency)
                .or_insert(vec![])
                .push(current_project.clone());
        }
    }

    dependency_map
}

fn update_dependency(dependency: Dependency, projects: &[String]) {
    for project in projects {
        Dotnet::update_package(project, &dependency.name, &dependency.latest).print_output();
    }
}

fn stringify_bytes(bytes: &[u8]) -> &str {
    std::str::from_utf8(bytes).unwrap()
}
