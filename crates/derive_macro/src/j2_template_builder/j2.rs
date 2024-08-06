use anyhow::Result;
use askama::Template;
use proc_macro::{TokenStream};
use crate::j2_template_builder::field_info::FieldInfo;
use crate::j2_template_builder::utils::{get_struct_fields, split};

/// 处理 jinja 模板的数据结构，builder.j2为模板，在模板中使用了 name / builder_name / fields
#[derive(Template)]
#[template(path = "builder.j2", escape = "none")]
pub struct BuilderContext {
    name: String,
    builder_name: String,
    fields: Vec<FieldInfo>,
}

impl BuilderContext {
    /// 从 TokenStream 中提取信息，构建 BuilderContext
    fn new(input: TokenStream) -> Self {
        let (name, input) = split(input);
        let fields = get_struct_fields(input);

        Self {
            builder_name: format!("{}Builder", name),
            name: name.to_string(),
            fields,
        }
    }

    /// 把模板渲染成字符串代码
    pub fn render(input: TokenStream) -> Result<String> {
        let template = Self::new(input);
        Ok(template.render()?)
    }
}


