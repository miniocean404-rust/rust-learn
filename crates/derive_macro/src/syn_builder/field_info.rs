use proc_macro2::Ident;
use syn::{Field, GenericArgument, Path, Type, TypePath};

/// 我们需要的描述一个字段的所有信息
struct FieldInfo {
    name: Ident,
    ty: Type,
    optional: bool,
}

/// 把一个 Field 转换成 Fd
impl From<Field> for FieldInfo {
    fn from(f: Field) -> Self {
        let (optional, ty) = crate::syn_builder::builder::get_option_inner(&f.ty);
        Self {
            // 此时，我们拿到的是 NamedFields，所以 ident 必然存在
            name: f.ident.unwrap(),
            optional,
            ty: ty.to_owned(),
        }
    }
}

// 如果是 T = Option<Inner>，返回 (true, Inner)；否则返回 (false, T)
fn get_option_inner(ty: &Type) -> (bool, &Type) {
    // 首先模式匹配出 segments
    if let Type::Path(TypePath {
                          path: Path { segments, .. },
                          ..
                      }) = ty
    {
        if let Some(v) = segments.iter().next() {
            if v.ident == "Option" {
                // 如果 PathSegment 第一个是 Option，那么它内部应该是 AngleBracketed，比如 <T>
                // 获取其第一个值，如果是 GenericArgument::Type，则返回
                let t = match &v.arguments {
                    syn::PathArguments::AngleBracketed(a) => match a.args.iter().next() {
                        Some(GenericArgument::Type(t)) => t,
                        _ => panic!("不确定如何处理其他GenericArgument"),
                    },
                    _ => panic!("不确定如何处理其他PathArguments"),
                };
                return (true, t);
            }
        }
    }
    (false, ty)
}