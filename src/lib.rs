extern crate num_cpus;
pub mod filter;
pub mod collection;


#[cfg(test)]
mod tests {
    use filter::{
        prime_filter,
        // prime_filter_section,
        old_prime_filter,
        prime_filter_section_sequentially,
    };
    extern crate time;
    extern crate num_cpus;
    use self::time::PreciseTime;
    #[test]
    fn test_prime_filter(){
        let n = 12345;
        let new_pf = prime_filter(n);
        let old_pf = old_prime_filter(n);
        for i in 0..n{
            assert_eq!(new_pf[i], old_pf[i], "Failed on number {}", i)
        }
    }
    use collection::{
        primes,
        primes_section,
        primes_concurrently,
        primes_section_sequentially,
    };
    #[test]
    fn test_prime_collection(){
        let some_primes = [ 2, 3, 5, 7, 11, 13,
                            17, 19, 23, 29, 31,
                            37, 41, 43, 47, 53,
                            59, 61, 67, 71, 73,
                            79, 83, 89, 97, 101,
                            103, 107, 109, 113, 127,
                            131, 137, 139, 149, 151,
                            157, 163, 167, 173, 179,
                            181, 191, 193, 197, 199];

        let test_primes: Vec<usize> = primes(200);
        for i in 0..test_primes.len(){
            assert_eq!(test_primes[i], some_primes[i], "Mismatch")
        };
    }
    #[test]
    fn test_prime_sec_collection(){
        let some_primes = [ 101,
                            103, 107, 109, 113, 127,
                            131, 137, 139, 149, 151,
                            157, 163, 167, 173, 179,
                            181, 191, 193, 197, 199];

        let test_primes: Vec<usize> = primes_section(100, 200);
        for i in 0..test_primes.len(){
            assert_eq!(test_primes[i], some_primes[i], "Mismatch")
        };
    }
    #[test]
    fn can_take_any_section_filter(){
        let some_primes = old_prime_filter(200);
        for min in 0..200{
            for max in (min+1)..200{
                for (i, is_prime) in prime_filter_section_sequentially(min, max).into_iter().enumerate(){
                    assert_eq!(some_primes[min+i], is_prime, "bad case from {} to {}", min, max);
                }
            }
        }
    }
    #[test]
    fn can_take_any_section(){
        let some_primes = old_prime_filter(200);
        for min in 0..200{
            for max in (min+1)..200{
                for prime in primes_section_sequentially(min, max).into_iter(){
                    assert!(some_primes[prime], "bad case from {} to {}: prime: {}", min, max, prime);
                }
            }
        }
    }
    #[test]
    fn count_primes(){
        let n = 100_000_000;
        let start = PreciseTime::now();
        let primes = primes(n);
        let end = PreciseTime::now();
        println!("{} seconds to find all primes less than {}!", start.to(end), n);
        println!("{} primes total!", primes.len());
        let total_primes = primes.len();
        assert_eq!(total_primes, 5761455);
    }
    #[test]
    fn count_primes_concurrently(){
        let threads = num_cpus::get()*4;
        let n = 1_000_000_000;
        let start = PreciseTime::now();
        let primes = primes_concurrently(n, threads);
        let end = PreciseTime::now();
        println!("{} seconds to find all primes less than {}! (Threads: {})", start.to(end), n, threads);
        println!("{} primes total!", primes.len());
        let total_primes = primes.len();
        assert_eq!(total_primes, 50_847_534);
    }
    #[test]
    fn count_prime_section(){
        let min = 1_000_000;
        let max = 10_000_000;
        let primes = primes_section(min, max);
        let total_primes = primes.len();
        assert_eq!(total_primes, 586081);
    }

}
