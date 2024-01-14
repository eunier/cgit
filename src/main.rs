use clap::{command, Arg, ArgMatches};
use regex::Regex;
use std::process::Command;

struct RepoInfo {
	remote: String,
	path: String,
}

fn main() {
	let remote_arg = Arg::new("remote");
	let matches = command!().arg(remote_arg).get_matches();
	let git_command = get_project_path(matches);
	run_git_command(git_command);
}

fn run_git_command(repo_info_option: Option<RepoInfo>) {
	match (repo_info_option, dirs::home_dir()) {
		(Some(repo), Some(home_dir)) => {
			let home_dir_str = home_dir.to_string_lossy();
			// Command to execute: echo Hello, Rust!
			let full_path = format!("{}/Projects/{}", home_dir_str, repo.path);

			let command = Command::new("git")
				.arg("clone")
				.arg(repo.remote)
				.arg(full_path)
				.spawn();

			// Check if the command was executed successfully
			match command {
				Ok(mut child) => {
					// Wait for the command to finish and get the result
					let status =
						child.wait().expect("Failed to wait for command");

					if status.success() {
						println!("Command executed successfully");
					} else {
						println!("Command failed with exit code: {}", status);
					}
				}
				Err(err) => {
					eprintln!("Failed to execute command: {}", err);
				}
			}
		}
		_ => {
			println!("Got None");
		}
	}
}

fn get_project_path(matches: ArgMatches) -> Option<RepoInfo> {
	if let Some(remote) = matches.get_one::<String>("remote") {
		if remote.starts_with("git") {
			let ssh_remote_regex = Regex::new(
				r"^git@([a-zA-Z0-9.-]+):([a-zA-Z0-9_/-]+)/([a-zA-Z0-9_.-]+)\.git$",
			)
			.unwrap();

			if ssh_remote_regex.is_match(remote) {
				let captures = ssh_remote_regex.captures(remote).unwrap();
				let hostname = captures.get(1).unwrap().as_str();
				let username = captures.get(2).unwrap().as_str();
				let repo = captures.get(3).unwrap().as_str();

				let hostname_parts: Vec<&str> = hostname.split('.').collect();
				let hostname_parts_reversed: Vec<&str> =
					hostname_parts.into_iter().rev().collect();
				let hostname_reversed: String =
					hostname_parts_reversed.join(".");

				let project_folder_name = repo.replace('-', "_");

				let root_folder_name =
					format!("{}.{}.{}", hostname_reversed, username, repo);

				let full_path =
					format!("{}/{}", root_folder_name, project_folder_name);

				return Some(RepoInfo {
					path: full_path,
					remote: remote.to_string(),
				});
			}
		}

		if remote.starts_with("https") {
			let ssh_remote_regex = Regex::new(
				r"^https://([a-zA-Z0-9.-]+)/([a-zA-Z0-9_/-]+)/([a-zA-Z0-9_.-]+)\.git$",
			)
			.unwrap();

			if ssh_remote_regex.is_match(remote) {
				let captures = ssh_remote_regex.captures(remote).unwrap();
				let hostname = captures.get(1).unwrap().as_str();
				let username = captures.get(2).unwrap().as_str();
				let repo = captures.get(3).unwrap().as_str();

				let hostname_parts: Vec<&str> = hostname.split('.').collect();
				let hostname_parts_reversed: Vec<&str> =
					hostname_parts.into_iter().rev().collect();
				let hostname_reversed: String =
					hostname_parts_reversed.join(".");

				let project_folder_name = repo.replace('-', "_");

				let root_folder_name =
					format!("{}.{}.{}", hostname_reversed, username, repo);

				let full_path =
					format!("{}/{}", root_folder_name, project_folder_name);

				return Some(RepoInfo {
					path: full_path,
					remote: remote.to_string(),
				});
			}
		}
	}

	None
}
