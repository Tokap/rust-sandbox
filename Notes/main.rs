#[allow(dead_code)]
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
  let summed_vector: usize = cube_and_sum(v);
  divisible_by_two(summed_vector)
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

fn split_to_num_vec(s: &str) -> Vec<usize> {
    let vec = s.split("").collect();
    let cropped = remove_head_and_tail(vec);
    map_int(cropped)
}

fn shift_vector(v: Vec<usize>) -> Vec<usize> {
    let mut vec = v.clone();
    let first_el = vec.remove(0);
    vec.push(first_el);
    vec
}

// This FN will likely be used to check data chunks
fn reverse_or_return(s: &str) -> Vec<usize> {
    let nums = split_to_num_vec(s.clone());

    if sum_and_check(nums.clone()) {
        nums.into_iter().rev().collect()
    } else {
        shift_vector(nums)
    }
}

fn main() {
    let trial = "123456";
    let trial_3 = "66443876";
    let my_outcome = reverse_or_return(trial_3);
    let my_outcome_2 = reverse_or_return(trial);

    let v = vec![1,2,3,4,5,6];
    let shift = shift_vector(v);

    println!("Outcome is {:?}", my_outcome);
    println!("Outcome Dos is {:?}", my_outcome_2);
    println!("Shift is {:?}", shift);

}
