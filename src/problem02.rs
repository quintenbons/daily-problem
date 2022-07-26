/// Hard - Uber
/// Multiplication over an array

#[cfg(test)]
mod tests {
    use super::*;

    fn general_test(f: fn(&Vec<i32>) -> Vec<i32>) {
        let before = vec![1, 2, 3, 4, 5];
        let res = f(&before);
        let expected = vec![120, 60, 40, 30, 24];

        assert_eq!(expected.len(), res.len());

        for i in 0..expected.len() {
            assert_eq!(expected.get(i), res.get(i));
        }
    }

    #[test]
    fn division_test() {
        general_test(mult_divide);
    }

    #[test]
    fn naive_test() {
        general_test(mult_naive);
    }

    #[test]
    fn no_divition_test() {
        general_test(mult_optimal);
    }
}

/// Gets the mult matrix of v by using division
fn mult_divide(v: &Vec<i32>) -> Vec<i32> {
    let mut product: i32 = v.iter().product();
    let mut res: Vec<i32> = Vec::with_capacity(v.len());

    for num in v {
        res.push(product / num);
    }

    res
}

/// Without division, naive
fn mult_naive(v: &Vec<i32>) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(v.len());

    for i in 0..v.len() {
        let mut to_add = 1;

        for j in 0..v.len() {
            if i == j {
                continue;
            }

            to_add *= v[j];
        }

        res.push(to_add);
    }

    res
}

/// Without division, optimal. This generates
/// suffixes and prefixes
fn mult_optimal(v: &Vec<i32>) -> Vec<i32> {
    let mut prefix: Vec<i32> = Vec::with_capacity(v.len() + 1);
    let mut suffix: Vec<i32> = Vec::with_capacity(v.len() + 1);
    let mut product = 1;

    // generate prefixes and suffixes
    prefix.push(1);
    for num in v {
        product *= num;
        prefix.push(product);
    }

    product = 1;
    suffix.push(1);
    for num in v.iter().rev() {
        product *= num;
        suffix.push(product);
    }
    suffix.reverse();


    // calculate result
    let mut res = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        res.push(prefix.get(i).unwrap() * suffix.get(i+1).unwrap());
    }

    res
}
