#![allow(unused_variables)]

/// 从代码设计角度来看，关于文件操作的类型和函数应该组织在一起，
/// 散落得到处都是，是难以管理和使用的。
///
/// 而且通过 open(&mut f1) 进行调用，
/// 也远没有使用 f1.open() 来调用好，
/// 这就体现出了只使用基本类型的局限性：无法从更高的抽象层次去简化代码。
type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

/// 发散函数
/// unimplemented!() 告诉编译器该函数尚未实现，
/// unimplemented!() 标记通常意味着我们期望快速完成主要代码，
/// 回头再通过搜索这些标记来完成次要代码，类似的标记还有 todo!()，
/// 当代码执行到这种未实现的地方时，程序会直接报错。
/// 你可以反注释 read(&mut f1, &mut vec![]); 这行，然后再观察下结果。
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let functions: Vec<(&str, Box<dyn Fn()>)> = vec![
        ("基本示例", Box::new(|| basic())),
    ];

    for (name, function) in functions.into_iter() {
        println!();
        println!(">>>>>>>>>>开始执行：{}", name);
        function();
        println!("{}: 执行结束<<<<<<<<<<", name);
    }
}


fn basic() {
    let mut f = File::from("file1.txt");
    open(&mut f);
    // read(&mut f, &mut vec![]);
    close(&mut f);
}