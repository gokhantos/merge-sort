#[allow(unused_variables)]
#[allow(unused_mut)]
mod merge_sort;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    thread, borrow::Borrow,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}




fn main() {
    let numbers = lines_from_file("numbers.txt");
    let mut vint: Vec<i32> = Vec::new();
    for i in &numbers{
        let num = i.parse::<i32>().unwrap();
        vint.push(num);
    }
    let mut _arr = &'static vint;
    let mut handles = Vec::new();
    let chunk_size = _arr.len()/8;
    let mut chunks = &_arr.chunks_mut(chunk_size).into_iter().collect::<Vec<&mut [i32]>>();
    for chunk in chunks{
        let handle = thread::spawn(move || {
            let right = chunk.len().checked_sub(1).unwrap();
            merge_sort::merge_sort(&mut chunk.to_vec(), 0, right);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    
}