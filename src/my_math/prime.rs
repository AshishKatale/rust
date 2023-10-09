use std::{
    sync::{Arc, Mutex},
    thread,
};

pub fn is_prime(n: usize) -> bool {
    let mut prime_flag = true;
    if n < 2 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    }

    if (n - 1) % 6 == 0 || (n + 1) % 6 == 0 {
        for i in (3..(n / 3)).step_by(2) {
            if n % i == 0 {
                prime_flag = false;
                break;
            }
        }
    } else {
        prime_flag = false;
    }

    return prime_flag;
}

pub fn get_primes_in_range(low: usize, high: usize) -> usize {
    let mut primes = 0;

    let check_if_prime = |n: usize| {
        let mut prime = true;
        for i in 3..n / 3 {
            if n % i == 0 {
                prime = false;
                break;
            }
        }
        prime
    };

    if low <= 2 {
        primes += 2;
    } else if low <= 3 {
        primes += 1;
    }

    let mut i;
    if low < 6 {
        i = 6
    } else {
        i = low / 6 * 6
    }

    while i <= high {
        let j = i - 1;
        let k = i + 1;

        if j >= low && j <= high && check_if_prime(j) {
            primes += 1;
        }
        if k >= low && k <= high && check_if_prime(k) {
            primes += 1;
        }

        i += 6;
    }

    primes
}

fn get_primes_in_range2(low: usize, high: usize, total: Arc<Mutex<usize>>) {
    let mut primes = 0;

    let check_if_prime = |n: usize| {
        let mut prime = true;
        for i in 3..n / 3 {
            if n % i == 0 {
                prime = false;
                break;
            }
        }
        prime
    };

    if low <= 2 {
        primes += 2;
    } else if low <= 3 {
        primes += 1;
    }

    let mut i;
    if low < 6 {
        i = 6
    } else {
        i = low / 6 * 6
    }

    while i <= high {
        let j = i - 1;
        let k = i + 1;

        if j >= low && j <= high && check_if_prime(j) {
            primes += 1;
        }
        if k >= low && k <= high && check_if_prime(k) {
            primes += 1;
        }

        i += 6;
    }

    let mut t = total.lock().unwrap();
    *t += primes;
}

pub fn get_primes_in_range_parallel(low: usize, high: usize, jobs: usize) -> usize {
    let block = (high - low) / jobs;
    let mut handlers = vec![];
    let total = Arc::new(Mutex::new(0_usize));

    for i in 0..jobs {
        let total = Arc::clone(&total);
        handlers.push(thread::spawn(move || {
            get_primes_in_range2(low + i * block, low + (i + 1) * block, total);
        }));
    }
    for h in handlers {
        let _ = h.join();
    }

    return *total.lock().unwrap();
}

#[cfg(test)]
mod tests {
    use super::{get_primes_in_range, get_primes_in_range_parallel};

    #[test]
    fn primes_in_range() {
        let primes_till_100 = get_primes_in_range(0, 100);
        let primes_till_1000 = get_primes_in_range(0, 1000);
        let primes_20_to_100 = get_primes_in_range(20, 100);
        let primes_100_to_1000 = get_primes_in_range(100, 1000);
        assert_eq!(primes_till_100, 25);
        assert_eq!(primes_till_1000, 168);
        assert_eq!(primes_20_to_100, 17);
        assert_eq!(primes_100_to_1000, 143);
    }

    #[test]
    fn primes_in_range_parallel() {
        let primes_till_100 = get_primes_in_range_parallel(0, 100, 4);
        let primes_till_1000 = get_primes_in_range_parallel(0, 1000, 4);
        let primes_20_to_100 = get_primes_in_range_parallel(20, 100, 4);
        let primes_100_to_1000 = get_primes_in_range_parallel(100, 1000, 4);
        assert_eq!(primes_till_100, 25);
        assert_eq!(primes_till_1000, 168);
        assert_eq!(primes_20_to_100, 17);
        assert_eq!(primes_100_to_1000, 143);
    }
}
