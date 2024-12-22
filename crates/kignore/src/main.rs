use clap::{Arg, Command};
use console::style;
use eyre::{Context, ContextCompat};
use std::io::prelude::*;
use std::{env::current_dir, io::Read, path::PathBuf};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn main() -> eyre::Result<()> {
    let matches = Command::new("gitignore")
        .version("0.1")
        .author("Kasper J. Hermansen <contact@kjuulh.io>")
        .about("Easily ignore items and remove from git state")
        .long_about("git ignore is a utility tool for easily adding patterns to your .gitignore file.
Easily add patterns using `git ignore <pattern>` this will by default also help you remove committed code violating these patterns
                    ")
        .propagate_version(true)
        .arg(
            Arg::new("pattern")
                .help("the pattern you want to ignore")
                .long_help("the pattern you want to ignore in the nearest .gitignore file")
                .required(true),
        ).arg(
            Arg::new("log-level").long("log-level").help("choose a log level and get more messages").long_help("Choose a log level and get more message, defaults to [fatal]"))
        .get_matches();

    let log_level = match matches.get_one::<String>("log-level").map(|f| f.as_str()) {
        Some("off") => "off",
        Some("info") => "info",
        Some("debug") => "debug",
        Some("warn") => "warn",
        Some("error") => "error",
        _ => "error",
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(format!(
            "gitignore={}",
            log_level
        )))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let term = console::Term::stdout();

    let pattern = matches
        .get_one::<String>("pattern")
        .context("missing [pattern]")?;

    add_gitignore_pattern(term, pattern)
}

enum GitActions {
    AddPattern { gitignore_path: PathBuf },
    CreateIgnoreAndAddPattern { git_path: PathBuf },
}

fn add_gitignore_pattern(term: console::Term, pattern: &String) -> eyre::Result<()> {
    println!("git ignore: Add pattern");
    let curdir = current_dir().context(
        "could not find current_dir, you may not have permission to access that directory",
    )?;
    let actions = match search_for_dotgitignore(&curdir)? {
        // If we have an ignore path, make sure it is in a git repo as well
        GitSearchResult::GitIgnore(ignorepath) => match search_for_git_root(&curdir)? {
            GitSearchResult::Git(_gitpath) => GitActions::AddPattern {
                gitignore_path: ignorepath,
            },
            _ => return Err(eyre::anyhow!("could not find parent git directory")),
        },
        // Find the nearest git repo
        GitSearchResult::Git(gitpath) => {
            GitActions::CreateIgnoreAndAddPattern { git_path: gitpath }
        } // We will always have either above, or an error so we have no default arm
    };

    match actions {
        GitActions::AddPattern { gitignore_path } => {
            println!("Found existing {}", style(".gitignore").green());
            let mut gitignore_file = open_gitignore_file(&gitignore_path)?;
            let mut gitignore_content = String::new();
            gitignore_file
                .read_to_string(&mut gitignore_content)
                .context(format!(
                    "could not read file: {}",
                    gitignore_path.to_string_lossy()
                ))?;
            if gitignore_content.contains(pattern) {
                println!(
                    ".gitignore already contains pattern, {}",
                    style("skipping...").blue()
                );
                return Ok(());
            }

            println!("adding pattern to file");
            writeln!(gitignore_file, "{}", pattern).context("could not write contents to file")?;
            gitignore_file
                .sync_all()
                .context("failed to write data to disk")?;
        }
        GitActions::CreateIgnoreAndAddPattern { git_path } => {
            println!(
                "could not find {} file, creating one in the root of the git repository",
                style(".gitignore").yellow()
            );
            let mut gitignore_file = create_gitignore_file(git_path)?;
            println!("adding pattern to file");
            writeln!(gitignore_file, "{}", pattern).context("could not write contents to file")?;
            gitignore_file
                .sync_all()
                .context("failed to write data to disk")?;
        }
    }

    // TODO: Run git rm -r --cached --pathspec <pattern> on the .git root
    let output = std::process::Command::new("git")
        .arg("rm")
        .arg("-r")
        .arg("--cached")
        .arg("-f")
        .arg("--ignore-unmatch")
        .arg(pattern)
        .output()
        .context("could not process git remove from index command")?;
    String::from_utf8(output.stdout)?
        .lines()
        .chain(String::from_utf8(output.stderr)?.lines())
        .map(|l| {
            // make rm 'path' look nice
            if l.contains("rm") {
                if let Some((_, pruned_first)) = l.split_once("'") {
                    if let Some((content, _)) = pruned_first.rsplit_once("'") {
                        return content;
                    }
                }
            }

            l
        })
        .for_each(|l| println!("removed from git history: {}", style(l).yellow()));

    if !output.status.success() {
        return Err(eyre::anyhow!("failed to run git index command"));
    }

    term.write_line("git successfully removed files")?;

    Ok(())
}

fn create_gitignore_file(gitroot: PathBuf) -> eyre::Result<std::fs::File> {
    let mut ignore_path = gitroot.clone();
    if !ignore_path.pop() {
        return Err(eyre::anyhow!("could not open parent dir"));
    }
    ignore_path.push(".gitignore");
    let file = std::fs::File::create(ignore_path.clone()).context(format!(
        "could not create file at path: {}",
        ignore_path.to_string_lossy()
    ))?;

    Ok(file)
}

fn open_gitignore_file(gitignore: &PathBuf) -> eyre::Result<std::fs::File> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(gitignore)
        .context(format!(
            "could not create file at path: {}",
            gitignore.to_string_lossy()
        ))?;

    Ok(file)
}

enum GitSearchResult {
    GitIgnore(PathBuf),
    Git(PathBuf),
}

fn search_for_git_root(path: &PathBuf) -> eyre::Result<GitSearchResult> {
    if !path.is_dir() {
        return Err(eyre::anyhow!(
            "path is not a dir: {}",
            path.to_string_lossy()
        ));
    }

    let direntries = std::fs::read_dir(path)
        .context(format!("could not open dir: {}", path.to_string_lossy()))?;
    for direntry in direntries {
        let entry = direntry.context("could not access file")?;

        let file_name = entry.file_name().to_os_string();
        if file_name.to_str().context("could not convert to str")? == ".git" {
            return Ok(GitSearchResult::Git(entry.path()));
        }
    }

    let mut upwards_par = path.clone();
    if !upwards_par.pop() {
        return Err(eyre::anyhow!(
            "no parent exists, cannot check further, you may not be in a git repository"
        ));
    }

    search_for_git_root(&upwards_par)
}

fn search_for_dotgitignore(path: &PathBuf) -> eyre::Result<GitSearchResult> {
    if !path.is_dir() {
        return Err(eyre::anyhow!(
            "path is not a dir: {}",
            path.to_string_lossy()
        ));
    }

    let direntries = std::fs::read_dir(path)
        .context(format!("could not open dir: {}", path.to_string_lossy()))?;
    for direntry in direntries {
        let entry = direntry.context("could not access file")?;

        let file_name = entry.file_name().to_os_string();

        if file_name.to_str().context("could not convert to str")? == ".gitignore" {
            return Ok(GitSearchResult::GitIgnore(entry.path()));
        }
    }

    let direntries = std::fs::read_dir(path)
        .context(format!("could not open dir: {}", path.to_string_lossy()))?;
    for direntry in direntries {
        let entry = direntry.context("could not access file")?;

        let file_name = entry.file_name().to_os_string();

        if file_name.to_str().context("could not convert to str")? == ".git" {
            return Ok(GitSearchResult::Git(entry.path()));
        }
    }

    let mut upwards_par = path.clone();
    if !upwards_par.pop() {
        return Err(eyre::anyhow!(
            "no parent exists, cannot check further, you may not be in a git repository"
        ));
    }

    search_for_dotgitignore(&upwards_par)
}
