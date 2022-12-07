use std::{ io, collections::{HashMap} };
use crate::utils::{self };

fn build_structure(
    lines: &Vec<String>, 
    dirs: &mut HashMap<String, Vec<String>>,
    files: &mut HashMap<String, Vec<(i64,String)>>,
) {
    let mut lines_iter = lines.iter();
    let mut curr_dir : String = String::from("/");
    lines_iter.next(); // We already have the first cd
    let mut parent : HashMap<String, String> = HashMap::new();

    loop {
        let curr_line = lines_iter.next();
        if curr_line == None { break; }
        let curr_line = curr_line.unwrap();
        match curr_line {
            x if x == "$ cd .." => { curr_dir = parent.get(&curr_dir).unwrap().to_string() }
            x if x.starts_with("$ cd ") => { 
                let new_dir = curr_line.split_once("$ cd ").unwrap().1;
                let new_dir = curr_dir.to_owned() + "/" + new_dir;
        
                parent.insert(new_dir.to_string(), curr_dir.to_string());
                curr_dir = new_dir.clone(); 
                let new_dir_clone =  new_dir.clone(); 

                if !dirs.contains_key(&new_dir) { dirs.insert(new_dir, Vec::new()); }
                if !files.contains_key(&new_dir_clone) { files.insert(new_dir_clone, Vec::new()); }
            }
            x if x.starts_with("dir ") => {
                let new_dir = curr_line.split_once("dir ").unwrap().1;
                let new_dir = curr_dir.to_owned() + "/" + new_dir;
                
                let mut inside : Vec<String> = dirs
                    .get(&curr_dir.to_string()).unwrap().iter().map(|x| x.to_string()).collect();

                inside.push(new_dir.to_string());
                dirs.insert(curr_dir.to_string(), inside);
            },
            x if x == "$ ls" => { }, // Skip
            _ => {
                // Files/dirs
                let new_file = curr_line.split_once(" ").unwrap();
                let new_file : (i64, String) = (new_file.0.parse::<i64>().unwrap(), new_file.1.to_string());
                let mut inside : Vec<(i64, String)> = files
                    .get(&curr_dir.to_string()).unwrap().iter().map(|x| x.clone()).collect();

                inside.push(new_file);
                files.insert(curr_dir.to_string(), inside);
            }
        }
    }
}

fn get_dir_size(
    dir: String,
    dirs: &HashMap<String, Vec<String>>, 
    files: &HashMap<String, Vec<(i64,String)>>,
    memo: &mut HashMap<String, i64>,
) -> i64 {
    if memo.contains_key(&dir) { return *memo.get(&dir).unwrap() }

    let mut dir_size : i64 = 0;
    // First we get the files sizes
    let dir_files : Vec<(i64,String)> = files.get(&dir)
        .unwrap_or(&Vec::new()).iter()
        .map(|x| x.clone()).collect();

    for file in dir_files {  dir_size += file.0; }
    // Now we get the inner folder
    let dir_dirs : Vec<String> = dirs.get(&dir)
        .unwrap_or(&Vec::new()).iter()
        .map(|x| x.clone()).collect();

    for ldir in dir_dirs {  
        dir_size += get_dir_size(ldir.to_string(), dirs, files, memo)
    }

    memo.insert(dir, dir_size);
    return dir_size;
}

fn first_star(memo: &HashMap<String, i64>) -> i64 {
    return memo.values().filter(|x| **x <= 100000).sum::<i64>();
}

fn second_star(memo: &HashMap<String, i64>) -> i64 {
    let free_space = 70000000 - memo.get("/").unwrap();
    let needed_space = 30000000 - free_space;
    return *memo.values().filter(|x| **x >= needed_space).min().unwrap();
}

pub fn solve() -> Result<(), io::Error>{
    let lines : Vec<String> = utils::get_lines("day_7").collect();
    let mut dirs : HashMap<String, Vec<String>> = HashMap::from([("/".to_string(), Vec::new())]);
    let mut files : HashMap<String, Vec<(i64,String)>> = HashMap::from([("/".to_string(), Vec::new())]);
    let mut memo : HashMap<String, i64> = HashMap::new(); 

    build_structure(&lines, &mut dirs, &mut files);
    get_dir_size("/".to_string(), &dirs, &files, &mut memo);
        
    println!("First Star: {:?}", first_star(&memo));
    println!("Second Star: {:?}",second_star(&memo));

    return Ok(())
}