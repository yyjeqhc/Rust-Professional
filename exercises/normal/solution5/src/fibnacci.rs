pub fn fib(n: u32) -> u32 {
    // TODO: Implement the logic to calculate the nth Fibonacci number using matrix exponentiation
    if n==0 || n==1{
        return n;
    }
    let mut first = 0;
    let mut second = 1;
    let mut tmp = 0;
    let mut arr = Vec::with_capacity(n as usize);
    arr.push(first);
    arr.push(second);
    for i in 1..n {
        tmp = first + second;
        if tmp > n {
            break;
        }
        arr.push(tmp);
        first = second;
        second = tmp;
    }
    println!("{:?}",arr);
    arr.iter().filter(|&x| x % 2 != 0).sum()
}
pub fn odd_fibnacci_sum(threshold: u32) -> u32 {
    fib(threshold)
}
