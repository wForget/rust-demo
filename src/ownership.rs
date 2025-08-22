pub(crate) fn run() {

    // -----  所有权 ------
    // &str 字符串字面值，不可变
    let s = "test";
    // String 字符串
    let mut s = String::from("test");
    s.push_str(" test");
    println!("s: {s}");

    // move
    let a = String::from("test");
    let b = a;
    // println!("a: {a}"); // borrow of moved value: `a`
    println!("b: {b}");

    // clone，深度拷贝
    let a = String::from("test");
    let b = a.clone();
    println!("a: {a}, b: {b}");

    // copy on stack
    let a = 1;
    let b = a;
    println!("a: {a}, b: {b}");

    // function ownership
    fn take_ownership(s: String) { // s 进入作用域
        println!("str: {s}")
    } // s 移出作用域并调用 `drop` 方法，释放内存
    let s = String::from("test");
    take_ownership(s);
    // println!("{s}") // value moved
    fn makes_copy(i: i32) {
        println!("i: {i}");
    }
    let i = 1i32;
    makes_copy(i);
    println!("i: {i}");
    // function return
    fn gives_ownership() -> String {
        let s = String::from("test1"); // s 进入作用域
        s // 返回 s 并移出给调用函数
    }
    fn takes_and_gives_back(s: String) -> String {
        // s 进入作用域
        s // s 移出给调用函数
    }
    let s1 = gives_ownership();
    let s2 = String::from("test2");
    let s3 = takes_and_gives_back(s2); // s2 moved
    println!("s1: {s1}, s3: {s3}");
    // Tips：变量的所有权总是遵循相同的模式：将值赋给另一个变量时它会移动。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

    // 使用 Tuple 来传递所有权
    {
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();
            (s, length)
        }
        let s = String::from("test");
        let (s, len) = calculate_length(s);
        println!("s: {s}, len: {len}");
    }

    // ---- 引用（reference）和借用（borrowing）
    // Tips: 引用（reference）像一个指针，因为它是一个地址，我们可以由此访问储存于该地址的属于其他变量的数据。与指针不同，引用在其生命周期内保证指向某个特定类型的有效值。
    {
        fn calculate_length(s: &String) -> usize { // s 是 String 的引用
            s.len()
        } // s 离开作用域，由于 s 是引用它并不拥有引用值的所有权，所以这里什么也不会发生
        let s = String::from("test");
        let len = calculate_length(&s);
        println!("s: {s}, len: {len}")
    }
    // Tips: 与使用 & 引用相反的操作是 解引用（dereferencing），解引用符号 *
    // Tips：我们将创建一个引用的行为称为 借用（borrowing）
    {
        // 引用默认也是不可变
        // fn change(s: &String) {
        //     s.push_str("_end"); // cannot borrow `*s` as mutable, as it is behind a `&` reference
        // }

        fn change(s: &mut String) {
            s.push_str("_end");
        }
        let mut s = String::from("test"); // Cannot borrow immutable local variable `s` as mutable
        change(&mut s);
        println!("changed s: {s}");

        // 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用
        // cannot borrow `s` as mutable more than once at a time
        // let mut s = String::from("test");
        // let r1 = &mut s; // first mutable borrow occurs here
        // println!("r1: {r1}");
        // let r2 = &mut s; // second mutable borrow occurs here
        // println!("r1: {r1}, r2: {r2}");        // first borrow later used here
        let mut s = String::from("test");
        {
            let r1 = &mut s;
        }
        let r2 = &mut s;
    }
    // 垂悬引用（Dangling References）
    // Tips：在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针（dangling pointer）—— 指向可能已被分配给其他用途的内存位置的指针
    {
        // error: Missing lifetime specifier
        // fn dangle() -> &String {
        //     let s = String::from("test");  // s 进入作用域
        //     &s // 返回 s 的引用
        // } // s 离开作用域，释放内存
        // let s = dangle();
    }
    // Tips：在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // Tips：引用必须总是有效的



}