use proc_macro::TokenTree;

/// 描述 struct 的每个 field
#[derive(Debug, Default)]
pub struct FieldInfo {
    name: String,
    ty: String,
    optional: bool,
}

impl FieldInfo {
    /// name 和 field 都是通过冒号 Punct 切分出来的 TokenTree 切片
    pub fn new(name: &[TokenTree], ty: &[TokenTree]) -> Self {
        // 把类似 Ident("Option"), Punct('<'), Ident("String"), Punct('>) 的 ty
        // 收集成一个 String 列表，如 vec!["Option", "<", "String", ">"]
        let ty = ty
            .iter()
            .map(|v| match v {
                TokenTree::Ident(n) => n.to_string(),
                TokenTree::Punct(p) => p.as_char().to_string(),
                e => panic!("Expect ident, got {:?}", e),
            })
            .collect::<Vec<_>>();
        // 冒号前最后一个 TokenTree 是 field 的名字
        // 比如：executable: String,
        // 注意这里不应该用 name[0]，因为有可能是 pub executable: String
        // 甚至，带 attributes 的 field，
        // 比如：#[builder(hello = world)] pub executable: String
        match name.last() {
            Some(TokenTree::Ident(name)) => {
                // 如果 ty 第 0 项是 Option，那么从第二项取到倒数第一项
                // 取完后上面的例子中的 ty 会变成 ["String"]，optiona = true
                let (ty, optional) = if ty[0].as_str() == "Option" {
                    (&ty[2..ty.len() - 1], true)
                } else {
                    (&ty[..], false)
                };
                Self {
                    name: name.to_string(),
                    ty: ty.join(""), // 把 ty join 成字符串
                    optional,
                }
            }
            e => panic!("Expect ident, got {:?}", e),
        }
    }
}