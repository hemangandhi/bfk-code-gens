// TODO: probs not `pub`
pub fn prime_factors(num: &mut i8) -> HashMap<i8, i8> {
    // TODO: compile-time program for this?
    const primes: [i8] = [
        2, 3, 4, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
        89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
        181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
    ];
    let mut result: HashMap<i8, i8> = HashMap::new();
    let mut smallest_prime_idx = 0;
    while num > 1 {
        if smallest_prime_idx >= primes.len() {
            panic!("This should be impossible: an i8 should be divisible by one of the primes");
        }
        if num % primes[smallest_prime_idx] == 0 {
            num = num / primes[smallest_prime_idx];
            result
                .entry(primes[smallest_prime_idx])
                .and_modify(|c| c += 1)
                .or_insert(1);
        } else {
            smallest_prime_idx += 1;
        }
    }
    return result;
}
