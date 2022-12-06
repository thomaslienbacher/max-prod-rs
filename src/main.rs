use std::fmt::{Debug, Display};
use std::io::{stdout, Write};

use num::{Integer, Unsigned};
use num::traits::Num;
use num::traits::real::Real;
use rand::{Rng, thread_rng};
use rand::distributions::uniform::Uniform;
use rand::distributions::Distribution;


fn max_prod_brute_force<T: Num + Copy + PartialOrd + Display>(arr: &[T]) -> (usize, usize) {
    let mut max_prod = T::zero();
    let mut max = (0, 0);
    let n = arr.len();

    for i in 0..n {
        for j in i..n {
            let mut prod = T::one();

            for k in i..=j {
                prod = prod * arr[k];
            }

            if prod > max_prod {
                max_prod = prod;
                max = (i, j);
            }
        }
    }

    assert!(max.0 <= max.1);
    max
}

fn max_prod_brute_force_improved<T: Num + Copy + PartialOrd + Display>(arr: &[T]) -> (usize, usize) {
    let mut max_prod = T::zero();
    let mut max = (0, 0);
    let n = arr.len();

    for i in 0..n {
        let mut prod = T::one();
        for j in i..n {
            prod = prod * arr[j];

            if prod > max_prod {
                max_prod = prod;
                max = (i, j);
            }
        }
    }

    assert!(max.0 <= max.1);
    max
}

fn max_prod_fast_int<T: Num + Integer + Copy + Unsigned>(arr: &[T]) -> (usize, usize) {
    let mut max_prod = T::zero();
    let mut max = (0, 0);

    let n = arr.len();
    let mut current = (0, 0);
    let mut current_prod = T::zero();

    for i in 0..n {
        if arr[i] != T::zero() {
            if current_prod == T::zero() {
                current_prod = T::one();
                current.0 = i;
            }
            current_prod = current_prod * arr[i];
            current.1 = i;
        } else {
            current = (i, i);
            current_prod = T::zero();
        }

        if current_prod > max_prod {
            max = current;
            max_prod = current_prod;
        }
    }

    assert!(max.0 <= max.1);
    max
}

fn compress_dual<T: Real + Copy>(arr: &[T]) -> Vec<(T, usize, usize)> {
    let n = arr.len();
    let mut compressed = Vec::new();

    let mut tmp_prod = T::one();
    let mut start = 0;

    let mut tmp_max = T::zero();
    let mut tmp_max_idx = 0;

    while start < n && arr[start] < T::one()  {
        if arr[start] > tmp_max {
            tmp_max = arr[start];
            tmp_max_idx = start;
        }

        start += 1;
    }

    if start == n {
        return vec![(tmp_max, tmp_max_idx, tmp_max_idx)];
    }

    let mut smaller = arr[start] < T::one(); // true = compressing numbers smaller than one

    for i in start..n {
        if smaller {
            if arr[i] < T::one() {
                tmp_prod = tmp_prod * arr[i];
            } else {
                compressed.push((tmp_prod, start, i - 1));
                smaller = false;
                tmp_prod = arr[i];
                start = i;
            }
        } else {
            if arr[i] > T::one() {
                tmp_prod = tmp_prod * arr[i];
            } else {
                compressed.push((tmp_prod, start, i - 1));
                smaller = true;
                tmp_prod = arr[i];
                start = i;
            }
        }
    }

    if tmp_prod > T::one() {
        compressed.push((tmp_prod, start, n - 1));
    }

    assert!(!compressed.is_empty());

    compressed
}

fn max_prod_fast_real<T: Real + Copy + Debug>(arr: &[T]) -> (usize, usize) {
    let mut compressed = compress_dual(arr);
    //println!("compressed = {:?}", compressed);
    let mut current_max = compressed[0];

    while compressed.len() >= 3 {
        //println!("compressed = {:?}", compressed);
        let a = compressed.pop().unwrap(); // arr[n - 1]
        let b = compressed.pop().unwrap(); // arr[n - 2]
        let c = compressed.pop().unwrap(); // arr[n - 3]

        //println!("a = {:?}  b = {:?}  c = {:?}", a, b, c);

        let combined = (a.0 * b.0 * c.0, c.1, a.2);
        //println!("combined = {:?}", combined);

        if combined.0 > c.0 {
            compressed.push(combined);
        } else {
            compressed.push(c);
        }

        if combined.0 > current_max.0 {
            current_max = combined;
            //println!("new max (com) = {:?}", current_max);
        }
        if a.0 > current_max.0 {
            current_max = a;
            //println!("new max ( a ) = {:?}", current_max);
        }
        if c.0 > current_max.0 {
            current_max = c;
            //println!("new max ( c ) = {:?}", current_max);
        }
    }

    //println!("final = {:?}", current_max);

    (current_max.1, current_max.2)
}

fn prod<T: Num + Copy>(arr: &[T], i: usize, j: usize) -> T {
    let mut prod = T::one();
    for k in i..=j {
        prod = prod * arr[k];
    }

    prod
}

fn main() {
    let arr: Vec<f64> = vec![0.7677789417518834, 0.8933695534913264, 0.3914341615624717, 0.7672288709480366, 0.20364132732776996];

    println!("F = {:?}", arr);
    let (i, j) = max_prod_fast_real(&arr[..]);
    stdout().flush().unwrap();
    println!("F[{i} .. {j}] = {}", prod(&arr, i, j));

    let c = max_prod_brute_force_improved(&arr);
    println!("F[{} .. {}] = {}", c.0, c.1, prod(&arr, c.0, c.1));
}

#[test]
fn test_real_basic() {
    let mut arr = vec![0.1, 0.5, 13.0, 2.0, 0.1, 4.0, 6.0, 7.0, 8.0, 0.1, 0.2];
    assert_eq!(max_prod_fast_real(&arr), max_prod_brute_force_improved(&arr));
}

#[test]
fn test_random_real() {
    for _ in 0..1000 {
        let mut a: Vec<f64> = thread_rng().sample_iter(Uniform::new(0.0, 2.0)).take(100).collect();
        assert_eq!(max_prod_fast_real(&a), max_prod_brute_force_improved(&a));
    }
}

#[test]
fn test_random_real2() {
    for i in 1..200 {
        let mut a: Vec<f64> = thread_rng().sample_iter(Uniform::new(0.0, 2.0)).take(i / 2).collect();
        println!("a = {:?}", a);
        assert_eq!(max_prod_fast_real(&a), max_prod_brute_force_improved(&a));
    }
}

#[test]
fn test_random_int() {
    for _ in 0..500 {
        let mut a: Vec<u128> = thread_rng().sample_iter(Uniform::new_inclusive(0, 10)).take(50).collect();

        let n = a.len();
        a[Uniform::new(0, n).sample(&mut thread_rng())] = 0;

        assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
        assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
    }
}

#[test]
fn test_brute_force_basic() {
    let a = vec![1u32, 2, 3, 4];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (0, 3));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_brute_force_basic2() {
    let a = vec![0u32, 2, 3, 4];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (1, 3));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_brute_force_basic3() {
    let a = vec![0, 1u32, 0, 0];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (1, 1));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_brute_force_basic4() {
    let a = vec![0, 1u32, 0, 1];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (1, 1));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_brute_force_basic5() {
    let a = vec![0, 1u32, 0, 7, 0, 3];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (3, 3));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_brute_force_basic6() {
    let a = vec![4u32];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (0, 0));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}

#[test]
fn test_integer1() {
    let a: Vec<u128> = vec![4, 9, 7, 3, 4, 8, 9, 2, 0, 3, 9, 6, 9, 2, 0, 5, 7, 2,
                            5, 8, 9, 7, 1, 5, 2, 8, 3, 7, 5, 2, 7, 8, 3, 1, 5, 4, 6, 1, 2,
                            5, 3, 2, 4, 4, 4, 3, 1, 9, 4, 7, 9, 4, 5, 7, 5, 5, 7, 5, 8, 9];
    let max = max_prod_fast_int(&a);
    assert_eq!(max, (15, 59));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force(&a));
    assert_eq!(max_prod_fast_int(&a), max_prod_brute_force_improved(&a));
}


#[test]
fn test_real_brute_force_01() {
    let farr: Vec<f32> = thread_rng().sample_iter(Uniform::new(0.0, 1.0)).take(20).collect();
    println!("F = {:?}", farr);
    let (i, j) = max_prod_brute_force(&farr[..]);
    let p = prod(&farr, i, j);
    let m = farr.into_iter().fold(0.0, |a, b| b.max(a));
    println!("F[{i} .. {j}] = {} (max = {m})", p);
    assert_eq!(p, m);
}
