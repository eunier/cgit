use std::fmt::format;

use clap::{command, Arg, ArgMatches};
use regex::Regex;

fn main() {
	let remote_arg = Arg::new("remote");
	let matches = command!().arg(remote_arg).get_matches();
	process_args_matches(matches);
}

fn process_args_matches(matches: ArgMatches) {
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

				println!("res: {}", full_path);
			}
		}
	}
}

// #[cfg(test)]
// mod tests {
// 	use super::*;

// 	#[test]
// 	fn test_process_args_matches() {
// 		process_args_matches()
// 	}
// }
