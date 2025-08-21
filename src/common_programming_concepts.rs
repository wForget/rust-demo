pub(crate) fn run() {
    // ---- variables and constants ----
    // immutable variables
    let x = 1;
    // x = 2;

    // mutable variables
    let mut y = 2;
    y = 3;

    // constants
    const Z: i32 = 10;
    // Z = 20;

    // Shadowing
    let a = 1;
    let a = "test";

    // ---- data types ----
    // scalar:
    //   integers: i8, i16, i32, i64, i128, isize
    //   unsigned integers: u8, u16, u32, u64, u128, usize
    //   float: f32, f64
    //   boolean: bool
    //   character: char
    let i: i32 = 100_100_100;

    // overflow
    let a: u8 = 255;
    assert_eq!(a.wrapping_add(1), 0);
    assert!(a.checked_add(1).is_none());
    assert!(a.overflowing_add(1).1);
    assert_eq!(a.saturating_add(1), 255);

    // compound: tuple and array
    // tuple
    let tup: (i32, char) = (1, 'a');
    let (x, y) = tup;
    let x = tup.0;
    let y = tup.1;
    // array
    let arr: [i32; 2] = [1, 2];
    let arr = [3; 2];
    let [a ,b] = arr;
    println!("a: {}, b: {}", a, b);
    let a = arr[0];
    let b = arr[1];

    // function
    let c = add_func(1 ,2);

    // control
    // if
    let n = if a > 0 {
        1
    } else if a == 0 {
        0
    } else {
        -1
    };
    // loops
    let mut n = 0;
    let mut sum = n;
    let a = loop {
        n = n + 1;
        if n % 2 == 0 {
            continue;
        };
        sum += n;
        if sum > 100 {
            break n;
        }
    };
    println!("sum: {}, a: {}, n: {}", sum, a, n);
    // loop label
    let mut n = 0;
    'top: loop {
        n += 2;
        loop {
            n += 3;
            if n <= 10 {
                continue 'top;
            }
            if n > 10 {
                break 'top;
            }
        }
    }
    println!("n: {}", n);
    // while
    let mut n = 10;
    let mut sum = 0;
    while n > 0 {
        n -= 1;
        sum += n;
    }
    println!("sum: {}, n: {}", sum, n);
    // for in array
    let arr = [1, 4, 9];
    let mut sum = 0;
    for a in arr {
        sum += a;
    }
    println!("sum: {}", sum);
}

fn add_func(a: i32, b: i32) -> i32 {
    a + b
}