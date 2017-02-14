pub mod filter;
pub mod collection;

#[cfg(test)]
mod tests {
    use filter::{
        prime_filter,
        old_prime_filter,
    };
    #[test]
    fn test_prime_filter(){
        let n = 1234567;
        let new_pf = prime_filter(n);
        let old_pf = old_prime_filter(n);
        for i in 0..n{
            assert_eq!(new_pf[i], old_pf[i], "Failed on number {}", i)
        }
    }
    use collection::primes;
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
    fn count_primes(){
        let n = 1000000;
        let primes = primes(n);
        let total_primes = primes.len();
        assert_eq!(total_primes, 78498);
    }
}
