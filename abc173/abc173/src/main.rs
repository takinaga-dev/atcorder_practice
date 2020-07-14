fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

// bin全探索でc問題を解く
fn main() {
    // 入力
    let line:String = read();
    let (h,w,k): (i32, i32, i32) = parse_str(line, ' ');
    //println!("{}, {}, {}", h, w, k);

    let mut v: Vec<String> = Vec::new();
    let mut v1: Vec<char> = Vec::new();
    let mut v2: Vec<Vec<char>> = Vec::new();
    let mut count: i32 = 0;
    for _ in 0..h {
        let s: String = read();
        v1 = s.chars().collect();
        v2.push(v1);
    }
    // h + w桁のbit列を作成する
    // そのためには2 ** (h + w)が必要
    // hが2, wが3の場合2 ** 4 - 1 で11111を作成する 
    for i in 0..pow(2, h + w - 1) - 1 {
        if calc_black(v2.clone(), i, h as usize, w as usize, k) {
            count += 1;
        }
    }

    // 出力
    // countしたresultを出す
    println!("{}", count);
}

// https://github.com/rust-lang/rust/issues/43262
fn parse_str<T>(line: String, determinator: char) -> (T, T, T)  where T: std::str::FromStr, <T as std::str::FromStr>::Err : std::fmt::Debug
{
    let s: Vec<&str> = line.split(determinator).collect();
    let a: T = s[0].parse().unwrap();
    let b: T = s[1].parse().unwrap();
    let c: T = s[2].parse().unwrap();

    (a, b, c)
}

/*
numをkisu進数表示したときindex番目の桁数が1かどうかを判定する
例えば2進数で4桁が有効(1)の場合でそれを確認したいを考えると
is_keta_enable(9, 3, 2)で
num = 01001でこれを01000でANDを取る。
でこれの桁数を取得すると有効な桁数を取得できる。
*/
fn is_keta_enable(num: i32, index: i32, kisu: i32) -> bool {
    if calc_keta(num & 1 << (index - 1), kisu) != 0 {
        true
    } else {
        false
    }
}
// blocksと消す行と列のbit列を計算して#がk個ならtrueを返す
fn calc_black(mut blocks: Vec<Vec<char>>, num:i32, h: usize, w:usize, k: i32) -> bool {
    let mut result_count: i32 = 0;
    // 与えられたbit列について桁が1の場合特定の動作を実施する
    // 今回の場合1の場合、該当する列または行を赤く塗るという動作をする
    for i in 0..calc_keta(num, 2) {
        if !is_keta_enable(num, (i + 1) as i32, 2) {
            continue;
        }
        if i + 1 > 0 && i + 1 <= h {
            for j in 0..w {
                blocks[i][j] = 'r';
            }
        } else {
            for j in 0..h {
                //println!("h={}, w={}, index = {}, num = {}",j, index - h, index, num);
                blocks[j][i - h] = 'r';
            }
        }
    }

    // カウントを取る
    for i in 0..h {
        for j in 0..w {
            if blocks[i][j] == '#' {
                result_count += 1;
            }
        }
    }
    return result_count == k;
}

// baseのp乗
fn pow(base: i32, p: i32) -> i32 {
    let mut result: i32 = base;
    for _i in 0..p {
        result *= base;
    }

    result
}
// 10進数でnumが基数kisuで何桁の数字であるかを返す関数
fn calc_keta(mut num: i32, kisu: i32) -> usize {
    let mut count = 0;
    if num == 0 {
        return 0;
    }
    while num / kisu != 0{
        count += 1;
        num /= kisu;
    }
    count += 1;
    return count;
}