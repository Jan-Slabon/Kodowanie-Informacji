use bit_vec::BitVec;

extern crate bit_vec;


fn golomb_coding_naive(n : u32) -> BitVec
{
    let M : u32 = 4; // the divisor is 2 ^ M
    let mask : u32 = !(u32::max_value() << M);

    let quotient = n >> M;
    let mut reminder = n & mask;
    let mut code = BitVec::with_capacity(8);

    for _ in 1..quotient { code.push(true); }
    code.push(false);
    while reminder > 0 
    {
        code.push(reminder % 2 == 0);
        reminder = reminder >> 1;
    }
    return code;
}

fn golomb_decoding_naive(bv : BitVec) -> u32
{
    let mut quotient = 0;
    let mut reminder = 0;
    let mut it = bv.iter();
    while true == it.next().unwrap() {quotient = quotient + 1;}
    
    return 0;
}