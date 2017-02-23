// macro_rules! if_any_divides {
//      ( $( $div:tt ),* | $n:ident $if_b:block ) => { if $(($n % $div == 0))|* $if_b};
// }
// macro_rules! if_sm_prime_divides{
//     ( $n:ident $if_b:block ) => {if_any_divides!(
//     2, 3, 5, 7, 11, 13,
//     17, 19, 23, 29, 31,
//     37, 41, 43, 47, 53,
//     59, 61, 67, 71, 73,
//     79, 83, 89, 97, 101,
//     103, 107, 109, 113, 127,
//     131, 137, 139, 149, 151,
//     157, 163, 167, 173, 179,
//     181, 191, 193, 197, 199 | $n $if_b)};
// }
macro_rules! set_true_if_in_range{
    ( $( $i:tt ),* => $filter:ident + $offset:expr, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i-$offset] = true;})*
    };
    ( $( $i:tt ),* => $filter:ident, $min:expr, $max:expr) => {
        $(if ($min <= $i) & ($max > $i) {$filter[$i] = true;})*
    };
}

macro_rules! do_while{
    ( $do_:block $while_:expr ) => { while {$do_; $while_} {} }
}
// macro_rules! set_sm_primes_true {
//     ($filter:expr, $min:expr, $max:expr) => {
//         set_true_if_in_range!(
//         2, 3, 5, 7, 11, 13,
//         17, 19, 23, 29, 31,
//         37, 41, 43, 47, 53,
//         59, 61, 67, 71, 73,
//         79, 83, 89, 97, 101,
//         103, 107, 109, 113, 127,
//         131, 137, 139, 149, 151,
//         157, 163, 167, 173, 179,
//         181, 191, 193, 197, 199 => $filter + $min, $min, $max)
//     };
// }
