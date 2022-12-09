use std::collections::HashMap;

const SYSTEM_SPACE: usize = 70000000usize;
const UPDATE_SPACE: usize = 30000000usize;

#[derive(Debug, Clone)]
struct Directory<'a> {
    parent: Option<String>,
    directories: Vec<String>,
    files: Vec<(&'a str, usize)>,
}

fn main() {
    let input = include_str!("input.txt");

    let mut directories: HashMap<String, Directory> = HashMap::new();
    let mut directories_count: HashMap<&str, usize> = HashMap::new();

    build_directories(&mut directories, input);
    count_directories(&directories, &mut directories_count, "/");

    let first_count: usize = directories_count
        .values()
        .filter(|&&c| c <= 100000usize)
        .sum();

    let total_size: &usize = directories_count.get("/").unwrap();
    let free_space: usize = SYSTEM_SPACE - total_size;
    let needed_space: usize = UPDATE_SPACE - free_space;

    let second_count = directories_count
        .values()
        .filter(|&&c| c >= needed_space)
        .min()
        .unwrap();

    println!("{}", first_count);
    println!("{}", second_count);
}

fn build_directories<'a>(directories: &mut HashMap<String, Directory<'a>>, input: &'a str) {
    let mut parent: Option<String> = None;
    let mut actual_directory: Option<String> = None;

    input.split("$ ").skip(1).for_each(|command| {
        let mut lines = command.lines();
        let mut first_line = lines.next().unwrap().split_whitespace();

        match first_line.next().unwrap() {
            "cd" => {
                let directory = first_line.next().unwrap();
                match directory {
                    ".." => {
                        let dir = directories.get(parent.as_ref().unwrap()).unwrap();
                        (actual_directory, parent) = (parent.clone(), dir.parent.clone());
                    }
                    dir => {
                        let new_dir = match &actual_directory {
                            Some(act_dir) => {
                                let mut new_dir = String::from(act_dir);
                                new_dir.push_str("-");
                                new_dir.push_str(dir);

                                new_dir
                            }
                            None => String::from(dir),
                        };

                        [parent, actual_directory] = [actual_directory.clone(), Some(new_dir)];
                    }
                }
            }
            "ls" => {
                let directory = directories
                    .entry(actual_directory.clone().unwrap())
                    .or_insert(Directory {
                        parent: parent.clone(),
                        directories: Vec::new(),
                        files: Vec::new(),
                    });

                lines.for_each(|line| {
                    let (first, second) = line.split_once(" ").unwrap();
                    match first {
                        "dir" => {
                            let mut new_dir = String::from(actual_directory.clone().unwrap());
                            new_dir.push_str("-");
                            new_dir.push_str(second);

                            directory.directories.push(new_dir);
                        }
                        size => directory
                            .files
                            .push((second, size.parse::<usize>().unwrap())),
                    }
                })
            }
            _ => (),
        }
    });
}

fn count_directories<'a>(
    directories: &'a HashMap<String, Directory<'a>>,
    directories_count: &mut HashMap<&'a str, usize>,
    dir: &'a str,
) {
    let directory = directories.get(dir).unwrap();

    let file_size: usize = directory.files.iter().map(|(_, size)| size).sum();

    let dir_size: usize = directory
        .directories
        .iter()
        .map(|sub_dir| {
            if !directories_count.contains_key(&sub_dir[..]) {
                count_directories(directories, directories_count, &sub_dir[..]);
            }
            let &count = directories_count.get(&sub_dir[..]).unwrap();

            count
        })
        .sum();

    directories_count.insert(dir, file_size + dir_size);
}
