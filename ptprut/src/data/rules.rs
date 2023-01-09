#[macro_export]
// 转换字符串
macro_rules! go_str{
    ($e: expr) => {
        format!("{}", $e)
    };
}

#[macro_export]
// 数据库查询
macro_rules! go_search{
    ($e: expr, $f: expr, $g: expr) => {
        format!("select {} from {} where {}", $e, $f, $g)
    }
}