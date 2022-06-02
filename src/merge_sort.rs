
#[allow(unused_variables)]
#[allow(unused_mut)]
#[allow(non_snake_case)]
pub fn merge(v: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
    let mut n1 = mid.checked_sub(left).unwrap().checked_add(1).unwrap();
    let mut n2 = right.checked_sub(mid).unwrap();
    let mut L = vec![0; n1];
    let mut R = vec![0; n2];
    for i in 0..n1{
        L[i] = v[left + i];
    }
    for j in 0..n2{
        R[j] = v[mid + j + 1];
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < n1 && j < n2{
        if L[i] <= R[j]{
            v[k] = L[i];
            i += 1;
        }else{
            v[k] = R[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1{
        v[k] = L[i];
        i += 1;
        k += 1;
    }

    while j < n2{
        v[k] = R[j];
        j += 1;
        k += 1;
    }
}

pub fn merge_sort(v: &mut Vec<i32>, left: usize, right: usize) {
    if left < right{
        let mid = left.checked_add(right.checked_sub(left).unwrap()/ 2).unwrap();
        merge_sort(v, left, mid);
        merge_sort(v, mid.checked_add(1).unwrap() as usize, right as usize);
        merge(v, left, mid, right);
    }
}