// Original Idea:
fn square_sum(vec: Vec<i32>) -> i32 {
  let mut x = vec
  .into_iter()
  .map(|x| square(x))
  .into_iter()
  .sum();
  x
}

fn square(n: i32) -> i32 { n * n }

// Simple:
fn square_sum(vec: Vec<i32>) -> i32 {
    vec.into_iter()
        .map(|y| y * y)
        .fold(0, |a, b| a + b)
}

/*************************************/
/************************************/
/***********************************/

// Original Idea:
fn order(sentence: &str) -> String {
    if sentence == "" { "".to_string() }
    else { "hello".to_string() }
}

fn split_and_tuple(s: &str) -> Option<Vec<(String, i32)>> {
    let v: Vec<&str> = s.split(" ").collect();
    let mut th: Vec<_> = Vec::new();
    for w in v {
        let word_tuple: (String, i32) = make_tuple(w);
        th.push(word_tuple);
    }
    if th.len() > 1 { Some(th) }
    else { None }

}

fn make_tuple(s: &str) -> (String, i32) {
    let original = s.clone();
    let v: Vec<&str> = s.split("").collect();
    let mut n: &str = "0";
    for l in v {

        match l {
            "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => n = l,
            _ => continue,

        }
    }
    ( original.to_string(), n.parse::<i32>().unwrap() )
}

fn final_string(s: &str) -> String {
    let mut final_str = String::new();
    let x = s.clone().to_string();
    let mut vec = split_and_tuple(s).unwrap();
    vec.sort_by(|a,b| a.1.cmp(&b.1));
    for i in vec {
        final_str.push_str(&i.0);
        final_str.push_str(" ")
    }
    String::from(final_str.trim())
}

// Simple:
fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

//Simple 2:
fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

// Simple 3:
fn order(sentence: &str) -> String {
    let mut words = sentence.split_whitespace().collect::<Vec<&str>>();
    words.sort_by_key(|word| word.matches(char::is_numeric).next().unwrap());
    words.join(" ")
}
/*******************************************/
/*********** Mirror Functions *************/
/*****************************************/
fn hor_mirror(s: String) -> String {
    let mut v: Vec<&str> = s.rsplit("\n").collect();
    v.join("\n").to_string()
}

fn vert_mirror(s: String) -> String {
  let mut v: Vec<&str> = s.split("\n").collect();
  let mut z_iter = v.clone().into_iter();
  let mut revd : Vec<String> = Vec::new();

  for stringman in z_iter {
    let reversed = stringman.to_string().rsplit("").collect();
    revd.push(reversed);
  }
  revd.join("\n").to_string()
}

fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}
/*******************************************/
/**** Find Shortest Word in String ********/
/*****************************************/
// My Version
fn find_short(s: &str) -> u32 {
  let mut v: Vec<&str> = s.split(" ").collect();
  v.sort_by_key(|word| word.len());
  v[0].len() as u32
}

// Short Versions:
fn find_short(s: &str) -> usize {
  s.split_whitespace().map(str::len).min().unwrap()
}

// Version 2:
fn find_short(s: &str) -> u32 {
  s.split_whitespace()
   .map(|word| word.len())
   .min()
   .unwrap_or(0) as u32
}

/*********************************************/
/** Compare Arrays to see if A squared = B **/
/*******************************************/
// My Version
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if (a == []) && (b == []) { true }
    else if (a == []) || (b == []) { false }
    else {
        let mut a_sorted: Vec<i64> = a.clone();
        let mut b_sorted: Vec<i64> = b.clone();
        a_sorted.sort_by_key(|x| x.abs());
        b_sorted.sort_by_key(|x| x.abs());

        let a_final: Vec<i64> = a_sorted.into_iter().map(|n| n * n).collect();
        a_final == b_sorted
     }
}

// Small Version
fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>();
    let mut a2 = b;
    a1.sort();
    a2.sort();
    a1 == a2
}
