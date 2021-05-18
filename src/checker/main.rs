use synacor::memory::{MAX_ADDR, MAX_INT};

type Cache = Vec<Vec<Option<u16>>>;

fn main() {
    for h in 0..MAX_INT {
        let mut cache = vec![vec![None; MAX_ADDR]; 5];
        let res = check(&mut cache, 4, 1, h) % MAX_INT;
        println!("Running for {}, {}", h, res);
        if res % MAX_INT == 6 {
            println!("Sol: {}", res)
        }
    }
}

fn check(cache: &mut Cache, a: u16, b: u16, h: u16) -> u16 {
    let res = if let Some(v) = cache[a as usize][b as usize] {
        v
    } else {
        if a == 0 {
            (b + 1) % MAX_INT
        } else if b == 0 {
            check(cache, a - 1, h, h)
        } else {
            let b = check(cache, a, b - 1, h);
            check(cache, a - 1, b, h)
        }
    };
    cache[a as usize][b as usize] = Some(res);
    res
}
