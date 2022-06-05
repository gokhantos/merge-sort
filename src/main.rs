#[allow(unused_variables)]
#[allow(unused_mut)]

mod merge_sort;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,thread, borrow::{BorrowMut}, sync::{Mutex, Arc},
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
    let mut vint = Vec::new();
    for i in &numbers{
        let num = i.parse::<i32>().unwrap();
        vint.push(num);
    }
    use std::time::Instant;
    let mut handles = Vec::new();
    let max = vint.len();
    let chunk_size = max / 8;
    let m = Arc::new(Mutex::new(Vec::new()));
    let chunks = vint.chunks(chunk_size).map(|chunk| chunk.to_vec()).collect::<Vec<Vec<i32>>>();
    let now = Instant::now();
    for chunk in chunks{
        let num = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let right = chunk.len().checked_sub(1).unwrap();
            let mut chunk_vec = chunk.to_vec();
            merge_sort::merge_sort(chunk_vec.borrow_mut(), 0, right);
            num.lock().unwrap().append(&mut chunk_vec);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    let mut sorted_vecs = Arc::try_unwrap(m).unwrap().into_inner().unwrap();
    
    merge_sort::merge(sorted_vecs.borrow_mut(), 0, max/8-1, max/4-1);
    merge_sort::merge(sorted_vecs.borrow_mut(), max/4, max/4 + max/8-1, max/2-1);
    merge_sort::merge(sorted_vecs.borrow_mut(), max/2, max/2 + max/8-1, 3*max/4-1);
    merge_sort::merge(sorted_vecs.borrow_mut(), 3*max/4, 7*max/8-1, max-1);

    merge_sort::merge(sorted_vecs.borrow_mut(), 0, (max/2-1)/2, max/2-1);
    merge_sort::merge(sorted_vecs.borrow_mut(), max/2, max/2 + (max-1-max/2)/2, max-1);
    merge_sort::merge(sorted_vecs.borrow_mut(), 0, (max-1)/2, max-1);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("-----------------------------------------------------");
    let mut new_vec = vint.clone();
    let now = Instant::now();
    new_vec.sort();
    let elapsed = now.elapsed();
    println!("Elapsed 2: {:.2?}", elapsed);
    println!("{:?}", assert!(new_vec==sorted_vecs, "Sorted vectors are not equal"));
}