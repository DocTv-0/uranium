use quote::quote;
use proc_macro2::Span;
use syn::{parse_macro_input, Token, Result, parse::{Parse, ParseStream}, LitStr, token};

struct Nbt {
    values: Vec<NbtKeyValuePair>,
}

impl Nbt {
    fn to_compound(&self) -> Result<valence_nbt::Compound> {
        self.values
            .iter()
            .map(NbtKeyValuePair::to_entry)
            .collect()
    }
}

impl Parse for Nbt {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = Vec::new();
        while !input.is_empty() {
            values.push(input.parse::<NbtKeyValuePair>()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }
        Ok(Nbt { values })
    }
}

impl quote::ToTokens for Nbt {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let values = &self.values;
        tokens.extend(quote! {
            valence_nbt::compound! {
                #(
                    #values,
                )*
            }
        })
    }
}

enum NbtValue {
    Compound(Vec<NbtKeyValuePair>),
    List(Vec<NbtValue>),
    Expr(syn::Expr),
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ListType {
    String,
    Int,
    Bool,
    Float,
    Double,
}

impl Parse for NbtValue {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let mut values = Vec::new();
            while !content.is_empty() {
                values.push(content.parse::<NbtKeyValuePair>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }
            return Ok(NbtValue::Compound(values));
        } else if input.peek(token::Bracket) {
            let content;
            syn::bracketed!(content in input);
            let mut values = Vec::new();
            while !content.is_empty() {
                values.push(content.parse::<NbtValue>()?);
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                } else {
                    break;
                }
            }

            if let Some(first) = values.first() {
                let list_type = first.list_type();
                for value in values.iter().skip(1) {
                    if value.list_type() != list_type {
                        return Err(syn::Error::new(
                            value.span(),
                            "list elements must all have the same type",
                        ));
                    }
                }
            }

            return Ok(NbtValue::List(values));
        }
        Ok(NbtValue::Expr(input.parse()?))
    }
}

impl NbtValue {
    fn to_value(&self) -> Result<valence_nbt::Value> {
        match self {
            Self::Compound(values) => Ok(valence_nbt::Value::Compound(
                values
                    .iter()
                    .map(NbtKeyValuePair::to_entry)
                    .collect::<Result<valence_nbt::Compound>>()?,
            )),
            Self::List(values) => {
                let values = values
                    .iter()
                    .map(NbtValue::to_value)
                    .collect::<Result<Vec<_>>>()?;

                Ok(valence_nbt::Value::List(to_list(values, self.span())?))
            }
            Self::Expr(expr) => literal_expr_to_value(expr),
        }
    }

    fn list_type(&self) -> ListType {
        match self {
            NbtValue::Compound(_) | NbtValue::List(_) => ListType::String,
            NbtValue::Expr(expr) => match expr {
                syn::Expr::Lit(expr) => match &expr.lit {
                    syn::Lit::Bool(_) => ListType::Bool,
                    syn::Lit::Float(_) => ListType::Float,
                    syn::Lit::Int(_) => ListType::Int,
                    _ => ListType::String,
                },
                syn::Expr::Unary(expr) => match &expr.op {
                    syn::UnOp::Neg(_) => match &*expr.expr {
                        syn::Expr::Lit(expr) => match &expr.lit {
                            syn::Lit::Float(_) => ListType::Float,
                            syn::Lit::Int(_) => ListType::Int,
                            _ => ListType::String,
                        },
                        _ => ListType::String,
                    },
                    _ => ListType::String,
                },
                _ => ListType::String,
            },
        }

    }

    fn span(&self) -> Span {
        match self {
            NbtValue::Compound(_) | NbtValue::List(_) => Span::call_site(),
            NbtValue::Expr(expr) => syn::spanned::Spanned::span(expr),
        }
    }
}

fn to_list(values: Vec<valence_nbt::Value>, span: Span) -> Result<valence_nbt::List> {
    use valence_nbt::{List, Value};

    let Some(first) = values.first() else {
        return Ok(List::End);
    };

    macro_rules! collect_list {
        ($variant:ident) => {{
            values
                .into_iter()
                .map(|value| match value {
                    Value::$variant(value) => Ok(value),
                    _ => Err(syn::Error::new(span, "list elements must have the same type")),
                })
                .collect::<Result<Vec<_>>>()
                .map(List::$variant)
        }};
    }

    match first {
        Value::Byte(_) => collect_list!(Byte),
        Value::Short(_) => collect_list!(Short),
        Value::Int(_) => collect_list!(Int),
        Value::Long(_) => collect_list!(Long),
        Value::Float(_) => collect_list!(Float),
        Value::Double(_) => collect_list!(Double),
        Value::String(_) => collect_list!(String),
        Value::List(_) => collect_list!(List),
        Value::Compound(_) => collect_list!(Compound),
        Value::ByteArray(_) => collect_list!(ByteArray),
        Value::IntArray(_) => collect_list!(IntArray),
        Value::LongArray(_) => collect_list!(LongArray),
    }
}

fn literal_expr_to_value(expr: &syn::Expr) -> Result<valence_nbt::Value> {
    use valence_nbt::Value;
    use syn::spanned::Spanned;

    match expr {
        syn::Expr::Lit(expr) => match &expr.lit {
            syn::Lit::Bool(value) => Ok(Value::Byte(if value.value { 1 } else { 0 })),
            syn::Lit::Float(value) => {
                if value.suffix() == "f64" {
                    Ok(Value::Double(value.base10_parse()?))
                } else {
                    Ok(Value::Float(value.base10_parse()?))
                }
            }
            syn::Lit::Int(value) => match value.suffix() {
                "" | "i32" => Ok(Value::Int(value.base10_parse()?)),
                "i8" => Ok(Value::Byte(value.base10_parse()?)),
                "i16" => Ok(Value::Short(value.base10_parse()?)),
                "i64" => Ok(Value::Long(value.base10_parse()?)),
                suffix => Err(syn::Error::new(
                    value.span(),
                    format!("unsupported NBT integer suffix `{suffix}`"),
                )),
            },
            syn::Lit::Str(value) => Ok(Value::String(value.value())),
            literal => Err(syn::Error::new(
                literal.span(),
                "unsupported literal for NBT",
            )),
        },
        syn::Expr::Unary(expr) if matches!(expr.op, syn::UnOp::Neg(_)) => {
            let value = literal_expr_to_value(&expr.expr)?;
            match value {
                Value::Byte(value) => Ok(Value::Byte(value.checked_neg().ok_or_else(|| {
                    syn::Error::new(expr.span(), "integer literal overflows i8")
                })?)),
                Value::Short(value) => Ok(Value::Short(value.checked_neg().ok_or_else(|| {
                    syn::Error::new(expr.span(), "integer literal overflows i16")
                })?)),
                Value::Int(value) => Ok(Value::Int(value.checked_neg().ok_or_else(|| {
                    syn::Error::new(expr.span(), "integer literal overflows i32")
                })?)),
                Value::Long(value) => Ok(Value::Long(value.checked_neg().ok_or_else(|| {
                    syn::Error::new(expr.span(), "integer literal overflows i64")
                })?)),
                Value::Float(value) => Ok(Value::Float(-value)),
                Value::Double(value) => Ok(Value::Double(-value)),
                _ => Err(syn::Error::new(
                    expr.span(),
                    "only numeric literals can be negated",
                )),
            }
        }
        syn::Expr::Unary(expr) if matches!(expr.op, syn::UnOp::Not(_)) => {
            let Value::Byte(value) = literal_expr_to_value(&expr.expr)? else {
                return Err(syn::Error::new(
                    expr.span(),
                    "only boolean literals can be negated",
                ));
            };
            Ok(Value::Byte(if value == 0 { 1 } else { 0 }))
        }
        syn::Expr::Paren(expr) => literal_expr_to_value(&expr.expr),
        syn::Expr::Group(expr) => literal_expr_to_value(&expr.expr),
        expr => Err(syn::Error::new(
            expr.span(),
            "expected a literal NBT value",
        )),
    }
}

impl NbtKeyValuePair {
    fn to_entry(&self) -> Result<(String, valence_nbt::Value)> {
        Ok((self.key.clone(), self.value.to_value()?))
    }
}

impl quote::ToTokens for NbtValue {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            NbtValue::Compound(values) => {
                tokens.extend(quote! {
                    valence_nbt::compound! {
                        #(
                            #values,
                        )*
                    }
                })
            }
            NbtValue::List(values) => {
                let list_type = values
                    .first()
                    .map(NbtValue::list_type)
                    .unwrap_or(ListType::String);
                let list = match list_type {
                    ListType::String => quote! {
                        valence_nbt::List::String(vec![
                            #(
                                (#values).to_string(),
                            )*
                        ])
                    },
                    ListType::Int => quote! {
                        valence_nbt::List::Int(vec![
                            #(
                                (#values) as i32,
                            )*
                        ])
                    },
                    ListType::Bool => quote! {
                        valence_nbt::List::Byte(vec![
                            #(
                                (#values) as i8,
                            )*
                        ])
                    },
                    ListType::Float => quote! {
                        valence_nbt::List::Float(vec![
                            #(
                                (#values) as f32,
                            )*
                        ])
                    },
                    ListType::Double => quote! {
                        valence_nbt::List::Double(vec![
                            #(
                                (#values) as f64,
                            )*
                        ])
                    }
                };
                tokens.extend(quote! {
                    valence_nbt::Value::List(#list)
                })
            }
            NbtValue::Expr(expr) => {
                tokens.extend(quote! { #expr })
            }
        }
    }
}

struct NbtKeyValuePair {
    key: String,
    value: NbtValue,
}

impl Parse for NbtKeyValuePair {
    fn parse(input: ParseStream) -> Result<Self> {
        let key = input.parse::<LitStr>()?.value();
        input.parse::<Token![:]>()?;
        let value = input.parse()?;
        Ok(Self { key, value })
    }
}

impl quote::ToTokens for NbtKeyValuePair {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let key = LitStr::new(&self.key, Span::call_site());
        let value = &self.value;
        tokens.extend(quote! {
            #key => #value
        })
    }
}

impl Nbt {
    fn is_literal(&self) -> bool {
        self.values.iter().all(NbtKeyValuePair::is_literal)
    }
}

impl NbtKeyValuePair {
    fn is_literal(&self) -> bool {
        self.value.is_literal()
    }
}

impl NbtValue {
    fn is_literal(&self) -> bool {
        match self {
            Self::Compound(values) => {
                values.iter().all(NbtKeyValuePair::is_literal)
            }
            Self::List(values) => {
                values.iter().all(NbtValue::is_literal)
            }
            Self::Expr(expr) => is_literal_expr(expr),
        }
    }
}

fn is_literal_expr(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Lit(_) => true,

        // Supports values such as -42 and !true.
        syn::Expr::Unary(expr) => {
            matches!(expr.op, syn::UnOp::Neg(_) | syn::UnOp::Not(_))
                && is_literal_expr(&expr.expr)
        }

        // Supports `(42)` and grouped expressions.
        syn::Expr::Paren(expr) => is_literal_expr(&expr.expr),
        syn::Expr::Group(expr) => is_literal_expr(&expr.expr),

        _ => false,
    }
}

#[proc_macro]
pub fn nbt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if input.is_empty() {
        return quote! {
            Vec::<u8>::new()
        }.into();
    }
    let nbt = parse_macro_input!(input as Nbt);

    if nbt.is_literal() {
        let compound = match nbt.to_compound() {
            Ok(compound) => compound,
            Err(error) => return error.into_compile_error().into(),
        };
        let mut bytes = Vec::new();
        valence_nbt::to_binary(&compound, &mut bytes, "").unwrap();
        bytes.drain(1..3);

        quote! {{ vec![#(#bytes),*] }}.into()
    } else {
        quote! {{
            let nbt = #nbt;
            let mut bytes = Vec::new();
            valence_nbt::to_binary(&nbt, &mut bytes, "").unwrap();
            bytes.copy_within(3.., 1);
            bytes.truncate(bytes.len() - 2);
            bytes
        }}.into()
    }


}