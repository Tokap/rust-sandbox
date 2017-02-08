fn cube(c: usize) ->  usize {
    c * c * c
}

fn sum_vector(v: Vec<usize>) -> usize {
  v.iter().sum()
}

#[allow(dead_code)]
fn divisible_by_two(c: usize) -> bool {
  c % 2 == 0
}

fn cube_vector(v: Vec<usize>) -> Vec<usize> {
  v.clone().into_iter().map(|x| cube(x)).collect()
}

fn cube_and_sum(c: Vec<usize>) -> usize {
  let cubed_v: Vec<usize> = cube_vector(c);
  sum_vector(cubed_v)
}

fn sum_and_check(v: Vec<usize>) -> bool {
  let summed_vector: usize = sum_vector(v);
  divisible_by_two(summed_vector)
}

fn general_process(s: &str, c: usize) {

}

fn to_integer(s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn map_int(v: Vec<&str>) -> Vec<usize> {
    v.clone().into_iter().map(|x| to_integer(x)).collect()
}

fn remove_head_and_tail(v: Vec<&str>) -> Vec<&str> {
    let mut vec = v.clone();
    vec.remove(0);
    vec.pop();
    vec
}


fn main() {
    let vec: Vec<usize> = vec![1,2,3];

    let mut str_vec: Vec<&str> = vec!["", "1","2","3", ""];

    // let changed = map_int(str_vec);

    let cubed: Vec<usize> = cube_vector(vec);

    let v = vec![1,2,3,4,5,6];

    let v = v.into_iter().filter(|&i|i != 0).collect::<Vec<_>>();

    let cropped = remove_head_and_tail(str_vec);

    // let tester = sum_and_check(cubed);

    // println!("Sum is {:?}", changed);
    println!("Test is {:?}", cropped);
    // println!("OG is {:?}", str_vec);

}
