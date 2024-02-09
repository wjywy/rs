// 引入标准库中的env包
use std::env;
use std::process; // 引入process包，用于退出程序

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect(); // env::args读取到的第一个参数就是程序的可执行路径名

    // dbg!(args); // 加上此行报错，原因是试图使用一个已经被移动的值。一个值只能有一个所有者，除非你加上&，表示引用。引用允许引用一个已经存在的值，而不是创建一个新的副本

    // unwrap_or_else是Rust中Option和Result枚举的一个常用方法。它尝试获取枚举中包含的值（对于Result, 是OK变体中的值）。如果枚举是None(对于Option)，或Err(对于Result)，
    // 则执行传递给unwrap_or_else的闭包(即|err| {...}部分)
    // 在这个例子中，如果Config::build返回OK(config)，unwrap_or_else会提取出config并赋值给变量config
    // 如果返回Err(err),则执行闭包，打印错误信息，并通过process::exit(1)终止程序。这里err是Config::build方法返回的错误
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("query {}", config.query);
    println!("filePath {}", config.file_path);

    // fs::read_to_string会返回一个Result类型，它表示操作可能成功或者失败。如果操作成功，expect会返回OK中的值，
    // 如果操作失败，expect会打印传递给它的字符串（在这个例子中是“hello wujiayu”）

    // 只需要用if let去匹配是否发生错误即可
    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {e}");
        process::exit(1);
    };
    // println!("contents\n{contents}"); // 如果上方没有加expect，那么是不支持打印的，因为Result类型本身没有实现Display trait，而println!需要Display trait
}