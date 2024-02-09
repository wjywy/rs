use std::error::Error;
use std::fs;
use  std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // fn new 这是Config结构体的一个关联函数。
    pub fn build (mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments"); // 想要输出对用户友好的信息，使用panic是不够的，建议使用Result来代替panic
        // }
        args.next(); // 跳过第一个参数，因为它是程序名

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string."),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path."),
        };
        // let query = args[1].clone();
        // let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok(); // is_ok用于检查是否有值，有就返回true，否则返回false

        // 使用字段初始化语法创建一个新的Config实例。这里，query和file_path字段被初始化为之前从args切片中克隆的字符串
        Ok(Config {query, file_path, ignore_case,})
    }
}

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("contents\n{contents}");
    let result = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
    Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); // 这个断言表示两个表达式是相等的
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new(); // 表示result是可变的，否则不能进行增删查改； 可变引用: &mut

    // for line in contents.lines() { // lines，将目标字符串按行进行分割
    //     if line.contains(query) {
    //         result.push(line);
    //     }
    // }

    // result
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}