// 这是一个涵盖以下章节内容的测验：
// - 字符串
// - 向量
// - 移动语义
// - 模块
// - 枚举
//
// 让我们以函数的形式构建一个小型处理器。作为输入，我们将提供
// 一系列字符串和命令。这些命令决定了要对字符串执行的操作。
// 可能的操作包括：
// - 将字符串转换为大写
// - 修剪字符串的首尾空白
// - 将"bar"字符串重复指定次数并追加到原字符串后
//
// 具体形式如下：
// - 输入是一个包含两个元素的元组的向量，
//   第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串向量。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 待办：按上述要求完成此函数
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> { 
        let mut output = vec![];

        for cmd in input {
            let s = cmd.0;
            match cmd.1 {
                Command::Uppercase => output.push(s.to_uppercase()),
                Command::Trim => output.push(s.trim().to_string()),
                Command::Append(n) => output.push(format!("{}{}", s, "bar".repeat(n))),
            }
        }
        output
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // 待办：我们需要导入什么来使 transformer 在作用域内？
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
