macro_rules! do_if_mod_60_match_pat{
    ($($case_pattern:pat)|+, $n:ident < $max:ident, $operation:expr) =>{
                    match $n{
                        n if n >= $max => break,
                        n => {match n%60{
                            $($case_pattern)|+ => $operation,
                            _ => (),
                        };},
                    }
                }
}
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
