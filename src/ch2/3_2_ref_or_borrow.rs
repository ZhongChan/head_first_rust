fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
        ("不可变引用", Box::new(|| not_mut_ref())),
        ("可变引用", Box::new(|| mut_ref())),
        ("可变引用同时只有有一个", Box::new(|| only_one_mut_ref())),
        ("NLL", Box::new(|| nll())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}

fn basic() {
    let x = 5;
    let y = &x;
    assert_eq!(x, 5);
    // assert_eq!(y, 5);// can't compare `&{integer}` with `{integer}`
    assert_eq!(*y, 5);//解引用，获取值
}

fn not_mut_ref() {
    let s1 = String::from("hello");
    let l = cal_len(&s1);
    println!("{} length is {}", s1, l)
}

fn cal_len(some_string: &String) -> usize {
    some_string.len()
}

fn mut_ref() {
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(",world");
}

fn only_one_mut_ref() {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    // let s3 = &mut s1;
    // println!("s1:{},s2:{},s3:{}", s1, s2, s3); // cannot borrow `s1` as mutable more than once at a time
    println!("s2:{}", s2);

    let mut t1 = String::from("world");
    {
        let t2 = &mut t1;
        // println!("t1:{}", t1); // cannot borrow `t1` as immutable because it is also borrowed as mutable
        println!("t2:{}", t2);
    }
    let t3 = &mut t1;
    println!("t3:{}", t3);
}

// None-Lexical Lifetimes
fn nll() {
    let mut s1 = String::from("hello");
    let s2 = &s1;
    let s3 = &s1;
    //s2 s3 是多个不可变引用，且生命周期结束(最后一次使用)
    //println 只是对 s2 s3的借用
    println!("s2:{},s3:{}", s2, s3);


    let s4 = &mut s1; //只能由一个可变引用
    println!("s4:{}", s4);
}