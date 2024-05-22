pub fn fibbonaci_code(mut number : i32) -> Vec<i32>
{
    let mut code : Vec<i32> = Vec::<i32>::new();
    let mut first = 0;
    let mut count = 0;
    while number > 0
    {
        (first, count) = closest_fibbonaci(&number);
        number = number - first;
        code.push(count);
    }
    return code;
}

pub fn fibbonaci_decode(code : Vec<i32>) -> i32
{
    code.iter().fold(0, |acc, el|{acc+fib(*el)})
}
fn fib(n : i32) -> i32
{
    let mut first = 1;
    let mut second = 1;
    for _  in 1..n
    {
        second += first;
        first = second - first;
    }
    return second;
}
fn closest_fibbonaci(number : &i32) ->  (i32, i32)
{
    let mut first = 1;
    let mut second = 1;
    let mut count : i32 = 0;
    while second < *number 
    {
        count += 1;
        second = first + second;
        first = second - first;
    }
    return (first, count);
}
