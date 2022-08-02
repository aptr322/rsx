use url::Url;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    let prefx = &["www.", "www1.", "www2.", "l.", "m.", "lm."];

    for p in prefx {
        println!("{}", &p);
        if "www1.aaaa".starts_with(p) {
            break;
        }
    }
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

    println!("{:?}  {}", url, url.domain().unwrap());
    print_type_of(&url.path());
}

#[test]
fn t_3() {
    let v1 = (0..25).step_by(7).collect::<Vec<_>>();

    for c in v1 {
        println!("{:?}", c);
    }
}

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
    
    println!("{:?}", &v1);
}
