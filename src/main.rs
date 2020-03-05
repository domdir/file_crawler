use std::cmp::Ordering;
use std::env;
use std::io;
use walkdir::WalkDir;

#[derive(Eq)]
struct Entry {
    sec: u64,
    name: String,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sec.cmp(&other.sec)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.sec == other.sec
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("arg1: pathname");
        return Ok(());
    }

    let mut v = Vec::new();

    for entry in WalkDir::new(args.get(1).unwrap())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.metadata()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        let sec = sec.elapsed().unwrap().as_secs();

        v.push(Entry {
            name: name.to_string(),
            sec,
        })
    }

    v.sort();
    v.reverse();

    for entry in v.into_iter() {
        println!("{} \t {}", &entry.sec / (60 * 60 * 24), &entry.name);
    }

    let mut _input = String::new();
    let _ = io::stdin().read_line(&mut _input);

    Ok(())
}
