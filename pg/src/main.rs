#[allow(unused_imports)]
use url::Url;

mod nv;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


fn main() {
}


#[test]
fn t_1() {
    let o: Option<&str> = None;
    dbg!(o);

    dbg!(1 != 1);
}

#[test]
fn t_2() {
    let p = "https://moneywise.com/a/ch-c/americas-most-well-known-first-ladies-ranked/?utm_source=adwords_arz&utm_content=584278&utm_campaign=584278&azs=adwords_arz&utm_term=%7B%7Bpublisher_name%7D%7D_-_%7B%7Bsection_name%7D%7D_-_%7B%7Bpublisher_id%7D%7D_-_%7B%7Bsection_id%7D%7D&utm_medium=adwords_arz&gclid=Cj0KCQjwvLOTBhCJARIsACVldV2ynVpLtfcqTX3hNBXhFzkQYbEqLQjfN0V52AbJAFO5p8e7L80GCPQaAm5oEALw_wcB";

    let url = Url::parse(p).unwrap();

    println!("{:?}  {:?}  {:?}", url, url.domain().unwrap(), Some(url.path()));
    print_type_of(&url.path());
}

#[test]
fn t_3() {
    let v1 = (0..25).step_by(7).collect::<Vec<_>>();
    for c in v1 {
        println!("{:?}", c);
    }
}
#[allow(dead_code)]
fn contains(w: &str, discarded: &Vec<&str>) -> bool {
    for d in discarded {
        if *d == w {
            return true;
        }
    }
    false
}

#[test]
fn t_4() {
    let d = "lm.h.org";
    let discarded = vec!["www", "www1", "l", "www2", "m", "lm"];
    
    let v: Vec<&str> = d.split(".").collect();

    let v1: Vec<String> = if contains(v[0], &discarded) {
        v[1..].iter().map(|v| v.to_string()).rev().collect()
    } else {
        v.iter().map(|v| v.to_string()).rev().collect()
    };

    let l = v1.len();

    dbg!(std::cmp::max(0, l as i32 - 4));
    println!("{:?}", &v1);
}

#[test]
fn t_5() {
    use regex::Regex;
    let seps = Regex::new("[+/=_\\-)(]").unwrap();

    let p = "/a/ch-c/americas-most-(well)-known+first=ladies-ranked/";

    let words: Vec<_> = seps.split(p).into_iter().filter(|x| !x.is_empty()).collect();

    for w in words {
        print_type_of(&w);
        println!("{:?}", &w);
    }
}

#[test]
fn t_6() {
    use regex::Regex;
    let re1 = Regex::new(r"\w\.htm.?$").unwrap();

    println!("{:?}", re1.is_match("aaa.htm"));

    let re2 = Regex::new(r"\d+").unwrap();
    println!("{:?}", re2.is_match("aaaaa1"));
    println!("{:?}", re2.is_match("123_"));

    let re3 = Regex::new(r"^((\w+).)*(\w+)$").unwrap();
    println!("{:?}", re3.is_match("aaa-a.b1.com"));
}


use std::any::Any;
pub fn is_string(s: &(dyn Any + Send)) -> bool {
    s.is::<String>()
}

#[test]
fn t_7() {
    use std::str::FromStr;
    let v = u64::from_str("124").unwrap();
    println!("0x{:08x} {}", v, is_string(&v));

    let v = u64::from(true);
    println!("{:?}", v);

}

#[test]
fn t_8() {
    // format!
    let p1 = "p1";
    let p2 = "p2";
    let p3 = 10;
    let o = format!("{p1}-{p2}-{p3}");

    println!("{}", o);
}

#[test]
fn t_9() {
    println!("{}", rand::random::<f64>());
}
