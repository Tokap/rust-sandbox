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

/****************************************************/
/** Calc Time to New Car Purchase Based on Params **/
/**************************************************/
fn nb_months(old: i32, new: i32, saving: i32, perc: f64) -> (i32, i32) {
    let mut old_car: f64    = old.clone() as f64;
    let mut new_car: f64    = new.clone() as f64;
    let mut savings: f64    = 0.0;
    let mut perc_drop: f64  = perc.clone() * 0.01;
    let mut months: i32     = 0;

    while new_car > (savings + old_car) {
        months += 1;
        if (months % 2 == 0) && (months != 0) { perc_drop = perc_drop + (0.5 * 0.01) }
        savings = savings + saving.clone() as f64;
        old_car = old_car * ( (0.0 - perc_drop.clone()) + 1.0);
        new_car = new_car * ( (0.0 - perc_drop.clone()) + 1.0);
    }
    let final_savings = ((savings + old_car ) - new_car).round() as i32;
    let final_months = months.clone();

    (final_months, final_savings)
}

/****************************************************/
/** Reverse, Rotate & manipulate strings          **/
/**************************************************/
fn rot(s: &str) -> String {
    let v: Vec<&str> = s.rsplit("\n").collect();
    let mut revd : Vec<String> = Vec::new();
    v.join("\n").to_string();

    for stringdude in v.into_iter() {
      let reversed = stringdude.to_string().rsplit("").collect();
      revd.push(reversed);
    }
    revd.join("\n").to_string()
}

fn selfie(s: &str) -> String {
    let v: Vec<&str> = s.split("\n").collect();
    let mut return_vec : Vec<String> = Vec::new();
    v.join("\n").to_string();

    for stringdude in v.into_iter() {
      let mut stringman = stringdude.to_string();
      let stringposter = stringman.clone();
      let dots: &str = if stringposter.len() > 2 {"...."} else {".."};
      stringman.push_str(dots);

      return_vec.push(stringman);
    }
    return_vec.join("\n").to_string()
}

fn selfie_and_rot(s: &str) -> String {
    let st = s.clone();
    let v: Vec<&str> = s.rsplit("\n").collect();
    let mut revd : Vec<String> = Vec::new();
    v.join("\n").to_string();

    for stringdude in v.into_iter() {
      let reversed: String = stringdude.to_string().rsplit("").collect();
      let rev_copy = reversed.clone();
      let mut dots_added: String = if rev_copy.len() > 2 {"....".to_string()} else { "..".to_string()};
      dots_added.push_str(&reversed);
      revd.push(dots_added);
    }
    let rot_complete: String = revd.join("\n").to_string();
    let mut selfie: String = selfie(st);
    selfie.push_str("\n");
    selfie.push_str(&rot_complete);
    selfie
}

// I tested edge case with below. Normally a 4x4 (4 words, 4 breaks) setup
let s: &str = "pR\nKo";
let result: String = selfie_and_rot(s);
println!("Result: {}",result );

// Simple Solution:
fn rot(s: &str) -> String {
    s.chars().rev().collect::<String>() // Big takeaway - you can specify return type with collect
}
fn rep(s: &str, n: usize) -> String {
    std::iter::repeat(s).take(n).collect::<String>()
}
fn selfie_and_rot(s: &str) -> String {
    let newstr1 = s.split('\n')
      .collect::<Vec<&str>>()
      .iter().map(|u| [u.to_string(), rep(".", u.len())].join("") ).collect::<Vec<String>>()
      .join("\n");
    let newstr2 = rot(s).split('\n')
      .collect::<Vec<&str>>()
      .iter().map(|u| [rep(".", u.len()), u.to_string()].join("") ).collect::<Vec<String>>()
      .join("\n");
    [newstr1, "\n".to_string(), newstr2].join("")

}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}
