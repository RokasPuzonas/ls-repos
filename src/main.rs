use std::{env, collections::HashSet, fs};

fn main() {
	let mut search_dirs = if env::args().count() > 1 {
		env::args().skip(1).collect()
	} else {
		vec![".".into()]
	};

	search_dirs.reverse();

	let mut showed_repos = HashSet::new();
	while !search_dirs.is_empty() {
		let dir = search_dirs.pop().unwrap();

		for entry in fs::read_dir(dir).unwrap() {
			let path = entry.unwrap().path();
			if !path.is_dir() { continue; }
			let canonical = path.canonicalize().unwrap();
			if showed_repos.contains(&canonical) { continue; }

			if path.join(".git").is_dir() {
				println!("{}", path.display());
				showed_repos.insert(canonical);
			} else {
				search_dirs.push(path.to_str().unwrap().into());
			}
		}
	}
}
