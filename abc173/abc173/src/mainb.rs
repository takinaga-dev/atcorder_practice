fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

fn main() {
    let mut ac: i32 = 0;
    let mut tle: i32 = 0;
    let mut wa: i32 = 0;
    let mut re: i32 = 0;
    let n:i32 = read();
    for _ in 0..n {
        let result: String = read();
        match &*result {
            "AC" => ac+=1,
            "TLE" => tle+=1,
            "WA" => wa+=1,
            "RE" =>re+=1,
            _ => println!("not matched"),
        }
    }
    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);

}