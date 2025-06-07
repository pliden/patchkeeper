use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::format_ident;
use quote::quote;
use std::collections::HashMap;
use std::collections::HashSet;
use syn::AttrStyle;
use syn::Data;
use syn::DataEnum;
use syn::DataStruct;
use syn::DeriveInput;
use syn::Field;
use syn::Fields;
use syn::GenericArgument;
use syn::Ident;
use syn::ItemStruct;
use syn::LitChar;
use syn::LitStr;
use syn::PathArguments;
use syn::Type;
use syn::Variant;
use syn::parse::Error;
use syn::parse_macro_input;
use syn::parse_quote;
use syn::spanned::Spanned;

#[proc_macro_derive(CmdLine, attributes(cmdline))]
pub fn derive_cmdline(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_cmdline_enum_or_struct(&input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn derive_cmdline_enum_or_struct(input: &DeriveInput) -> Result<TokenStream2, Error> {
    match &input.data {
        Data::Enum(data) => derive_cmdline_enum(&input, data),
        Data::Struct(data) => derive_cmdline_struct(&input, data),
        Data::Union(_) => Err(cannot_derive(input.ident.span(), "union type")),
    }
}

fn derive_cmdline_enum(input: &DeriveInput, data: &DataEnum) -> Result<TokenStream2, Error> {
    let enumeration = enum_attrs(input, data)?;

    let mut match_variants = vec![];

    for variant in &enumeration.variants {
        let ident = &variant.ident;
        let name = &variant.name;
        let aliases = &variant.aliases;

        match_variants.push(match &variant.kind {
            VariantKind::Unit => quote! {
                #name #(| #aliases)* => Ok(Self::#ident),
            },
            VariantKind::Unnamed => quote! {
                #name #(| #aliases)* => {
                    context.add_command(#name);
                    Ok(Self::#ident(::cmdline::parse(context, ::cmdline::no_value())?))
                }
            },
        });
    }

    let ident = &enumeration.ident;
    let meta = &enumeration.meta;
    let help_usage = enum_help_usage(&enumeration);
    let help_options = enum_help_options(&enumeration);

    Ok(quote! {
        impl ::cmdline::CmdLine for #ident {
            fn parse(context: &mut ::cmdline::Context, value: ::std::ffi::OsString) -> Result<Self, ::cmdline::Error> {
                let _ = context;
                let value = ::cmdline::value_to_string(value)?;
                match value.as_str() {
                    #(#match_variants)*
                    value => Err(::cmdline::invalid_variant(#meta, value)),
                }
            }

            fn help_usage(buf: &mut String, meta: &str) {
                #help_usage
            }

            fn help_options(buf: &mut String, variants: &str) {
                #help_options
            }
        }
    })
}

fn derive_cmdline_struct(input: &DeriveInput, data: &DataStruct) -> Result<TokenStream2, Error> {
    let structure = struct_attrs(input, data)?;

    let mut declare_fields = vec![];
    let mut match_fields = vec![];
    let mut unwrap_fields = vec![];

    for field in &structure.fields {
        let ident = &field.ident;
        let arg_ident = &field.arg_ident;
        let outer_type = &field.outer_type;
        let kind = &field.kind;
        let short = &field.short;
        let long = &field.long;
        let help_error = &field.help_error;

        if !field.is_help {
            declare_fields.push(quote! {
                let mut #arg_ident: Option<#outer_type> = None;
            });
        }

        let match_field = match (short, long) {
            (Some(short), None) => quote! {
                ::cmdline::Short(#short)
            },
            (None, Some(long)) => quote! {
                ::cmdline::Long(#long)
            },
            (Some(short), Some(long)) => quote! {
                ::cmdline::Short(#short) | ::cmdline::Long(#long)
            },
            (None, None) => match kind {
                FieldKind::OptionOther | FieldKind::Other => quote! {
                    ::cmdline::Value(value) if #arg_ident.is_none()
                },
                _ => quote! {
                    ::cmdline::Value(value)
                },
            },
        };

        let value_field = match (short, long, kind) {
            (_, _, FieldKind::OptionUnit) | (None, None, _) => quote! {},
            _ => quote! {
                let value = context.parser_value()?;
            },
        };

        let parse_field = match kind {
            FieldKind::OptionUnit => quote! {},
            _ => quote! {
                let value = ::cmdline::parse(context, value)?;
            },
        };

        let assign_field = match kind {
            FieldKind::OptionUnit => match field.is_help {
                true => quote! {
                    return ::cmdline::help::<Self>(context.commands());
                },
                false => quote! {
                    #arg_ident = ::cmdline::make_option_arg(());
                },
            },
            FieldKind::OptionVec => quote! {
                #arg_ident = ::cmdline::make_option_vec_arg(#arg_ident, value);
            },
            FieldKind::OptionOther => quote! {
                #arg_ident = ::cmdline::make_option_arg(value);
            },
            FieldKind::Vec => quote! {
                #arg_ident = ::cmdline::make_vec_arg(#arg_ident, value);
            },
            FieldKind::Other => quote! {
                #arg_ident = ::cmdline::make_arg(value);
            },
        };

        match_fields.push(quote! {
            #match_field => {
                #value_field
                #parse_field
                #assign_field
            }
        });

        unwrap_fields.push(match kind {
            FieldKind::OptionUnit | FieldKind::OptionVec | FieldKind::OptionOther => {
                match field.is_help {
                    true => quote! {},
                    false => quote! {
                        #ident: ::cmdline::unwrap_option_arg(#arg_ident),
                    },
                }
            }
            _ => {
                quote! {
                    #ident: ::cmdline::unwrap_arg(#arg_ident, #help_error)?,
                }
            }
        });
    }

    let ident = &structure.ident;
    let conflicts_and_choices = struct_conflicts_and_choices(&structure);
    let help_usage = struct_help_usage(&structure);
    let help_options = struct_help_options(&structure);

    Ok(quote! {
        impl ::cmdline::CmdLine for #ident {
            fn parse(context: &mut ::cmdline::Context, value: ::std::ffi::OsString) -> Result<Self, ::cmdline::Error> {
                let _ = value;

                #(#declare_fields)*

                while let Some(arg) = context.parser_next()? {
                    match arg {
                        #(#match_fields)*
                        ::cmdline::Short(short) => return Err(::cmdline::invalid_short(short)),
                        ::cmdline::Long(long) => return Err(::cmdline::invalid_long(long)),
                        ::cmdline::Value(arg) => return Err(::cmdline::invalid_argument(arg)),
                    }
                }

                #conflicts_and_choices

                Ok(Self {
                    #(#unwrap_fields)*
                })
            }

            fn help_usage(buf: &mut String, meta: &str) {
                #help_usage
            }

            fn help_options(buf: &mut String, meta: &str) {
                #help_options
            }
        }

        impl #ident {
            pub fn from_args<I: IntoIterator<Item = ::std::ffi::OsString>>(bin_name: &str, args: I) -> Self {
                ::cmdline::from_args(bin_name, args)
            }

            pub fn from_args_no_exit<I: IntoIterator<Item = ::std::ffi::OsString>>(bin_name: &str, args: I) -> Result<Self, ::cmdline::Error> {
                ::cmdline::from_args_no_exit(bin_name, args)
            }

            pub fn from_env(bin_name: &str) -> Self {
                ::cmdline::from_env(bin_name)
            }

            pub fn from_env_no_exit(bin_name: &str) -> Result<Self, ::cmdline::Error> {
                ::cmdline::from_env_no_exit(bin_name)
            }
        }
    })
}

struct StructAttrs {
    ident: Ident,
    fields: Vec<FieldAttrs>,
}

fn struct_attrs(input: &DeriveInput, data: &DataStruct) -> Result<StructAttrs, Error> {
    let mut structure = StructAttrs {
        ident: input.ident.clone(),
        fields: vec![],
    };

    match &data.fields {
        Fields::Named(_) => {}
        Fields::Unnamed(_) => {
            return Err(cannot_derive(structure.ident.span(), "tuple struct types"));
        }
        Fields::Unit => {
            return Err(cannot_derive(structure.ident.span(), "unit struct types"));
        }
    };

    for field in &data.fields {
        structure.fields.push(field_attrs(field, false)?);
    }

    let help_struct: ItemStruct = parse_quote! {
        struct Help {
            #[cmdline(help = "Print available options")]
            help: Option<()>,
        }
    };

    for field in &help_struct.fields {
        structure.fields.push(field_attrs(field, true)?);
    }

    struct_check(&structure)?;

    Ok(structure)
}

fn struct_check(structure: &StructAttrs) -> Result<(), Error> {
    let mut has_vec_field = false;

    for field0 in &structure.fields {
        if matches!(field0.short, None) && matches!(field0.long, None) {
            if matches!(field0.kind, FieldKind::OptionUnit) {
                return Err(cannot_derive(
                    field0.ident.span(),
                    format!("field (cannot have positional field of type Option<()>)"),
                ));
            }

            if matches!(field0.help1, Some(_)) {
                return Err(cannot_derive(
                    field0.ident.span(),
                    format!("field (cannot have help on positional field)"),
                ));
            }
        }

        let is_vec_field = match field0.kind {
            FieldKind::OptionVec | FieldKind::Vec => true,
            _ => false,
        };

        if is_vec_field {
            if has_vec_field {
                return Err(cannot_derive(
                    field0.ident.span(),
                    format!("field (cannot have field after field of type Vec<_>)"),
                ));
            }

            has_vec_field = true;
        }

        for field1 in &structure.fields {
            if std::ptr::eq(field0, field1) {
                continue;
            }

            if field0.short.is_some() && field0.short == field1.short {
                if field1.is_help {
                    return Err(cannot_derive(
                        field0.ident.span(),
                        format!(
                            "field (short '-{}' used by both '{}' and built-in help)",
                            field0.short.unwrap(),
                            field0.ident.to_string()
                        ),
                    ));
                } else {
                    return Err(cannot_derive(
                        field1.ident.span(),
                        format!(
                            "field (short '-{}' used by both '{}' and '{}')",
                            field0.short.unwrap(),
                            field0.ident.to_string(),
                            field1.ident.to_string()
                        ),
                    ));
                }
            }

            if field0.long.is_some() && field0.long == field1.long {
                if field1.is_help {
                    return Err(cannot_derive(
                        field0.ident.span(),
                        format!(
                            "field (long '-{}' used by both '{}' and built-in help)",
                            field0.long.as_ref().unwrap(),
                            field0.ident.to_string()
                        ),
                    ));
                } else {
                    return Err(cannot_derive(
                        field1.ident.span(),
                        format!(
                            "field (long '--{}' used by both '{}' and '{}')",
                            field0.long.as_ref().unwrap(),
                            field0.ident.to_string(),
                            field1.ident.to_string()
                        ),
                    ));
                }
            }
        }
    }

    Ok(())
}

fn struct_conflicts_and_choices(structure: &StructAttrs) -> TokenStream2 {
    let mut conflicts: HashMap<&String, Vec<(&Ident, &String)>> = HashMap::default();
    let mut choices: HashMap<&String, Vec<(&Ident, &String)>> = HashMap::default();

    for field in &structure.fields {
        for conflict in &field.conflicts {
            conflicts
                .entry(conflict)
                .and_modify(|entry| entry.push((&field.arg_ident, &field.help_error)))
                .or_insert(vec![(&field.arg_ident, &field.help_error)]);
        }

        for choice in &field.choices {
            choices
                .entry(choice)
                .and_modify(|entry| entry.push((&field.arg_ident, &field.help_error)))
                .or_insert(vec![(&field.arg_ident, &field.help_error)]);
        }
    }

    let mut check = vec![];

    for (_, conflict) in conflicts.iter() {
        for (i, (ident0, help_error0)) in conflict.iter().enumerate() {
            for j in i + 1..conflict.len() {
                let (ident1, help_error1) = &conflict[j];

                check.push(quote! {
                    if #ident0.is_some() && #ident1.is_some() {
                        return Err(::cmdline::conflicting_arguments(#help_error0, #help_error1));
                    }
                });
            }
        }
    }

    for (_, choice) in choices.iter() {
        for (i, (ident0, help_error0)) in choice.iter().enumerate() {
            for j in i + 1..choice.len() {
                let (ident1, help_error1) = &choice[j];

                check.push(quote! {
                    if #ident0.is_some() && #ident1.is_some() {
                        return Err(::cmdline::conflicting_arguments(#help_error0, #help_error1));
                    }
                });
            }
        }

        let idents = choice.iter().map(|(ident, _)| ident);
        let help_error = choice.iter().map(|(_, help_error)| help_error);
        check.push(quote! {
            if #(#idents.is_none())&&* {
                return Err(::cmdline::missing_argument(&[#(#help_error),*]));
            }
        });
    }

    quote! {
        #(#check)*
    }
}

enum FieldKind {
    OptionUnit,
    OptionVec,
    OptionOther,
    Vec,
    Other,
}

struct FieldAttrs {
    ident: Ident,
    arg_ident: Ident,
    outer_type: Type,
    inner_type: Type,
    kind: FieldKind,
    short: Option<char>,
    long: Option<String>,
    meta: String,
    variants: Option<String>,
    conflicts: Vec<String>,
    choices: Vec<String>,
    is_help: bool,
    help0: String,
    help1: Option<String>,
    help_error: String,
}

fn field_type(ty: &Type) -> Result<(FieldKind, Type, Type), Error> {
    let outer_type = ty.clone();
    let (kind, inner_type) = if is_option_type(&outer_type) {
        let inner_type = first_generic_arg(&outer_type)?;
        if is_unit_type(&inner_type) {
            (FieldKind::OptionUnit, inner_type)
        } else if is_vec_type(&inner_type) {
            let inner_type = first_generic_arg(&inner_type)?;
            (FieldKind::OptionVec, inner_type)
        } else {
            (FieldKind::OptionOther, inner_type)
        }
    } else if is_vec_type(&outer_type) {
        let inner_type = first_generic_arg(&outer_type)?;
        (FieldKind::Vec, inner_type)
    } else {
        let inner_type = outer_type.clone();
        (FieldKind::Other, inner_type)
    };

    Ok((kind, outer_type, inner_type))
}

fn first_generic_arg(ty: &Type) -> Result<Type, Error> {
    match ty {
        Type::Path(path) => match path.path.segments.last() {
            Some(last_segment) => match &last_segment.arguments {
                PathArguments::AngleBracketed(data) => data
                    .args
                    .iter()
                    .filter_map(|arg| match arg {
                        GenericArgument::Type(ty) => Some(ty),
                        _ => None,
                    })
                    .next()
                    .map(|ty| ty.clone()),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
    .ok_or(cannot_derive(ty.span(), "argument type"))
}

fn is_unit_type(ty: &Type) -> bool {
    match ty {
        Type::Tuple(tup) => tup.elems.is_empty(),
        _ => false,
    }
}

fn is_option_type(ty: &Type) -> bool {
    is_path_type(ty, &["std", "option", "Option"])
}

fn is_vec_type(ty: &Type) -> bool {
    is_path_type(ty, &["std", "vec", "Vec"])
}

fn is_path_type(ty: &Type, segments: &[&str]) -> bool {
    match ty {
        Type::Path(path) => {
            let mut segments0 = segments.iter().rev();
            let mut segments1 = path.path.segments.iter().rev();

            loop {
                match (segments0.next(), segments1.next()) {
                    (Some(s0), Some(s1)) if *s0 == s1.ident.to_string().as_str() => {}
                    (Some(_), None) => return path.path.leading_colon.is_none(),
                    (None, None) => return true,
                    (_, _) => return false,
                }
            }
        }
        _ => false,
    }
}

fn field_short(ident: &Option<Ident>) -> Option<char> {
    normalize_ident(ident.as_ref().unwrap()).chars().next()
}

fn field_long(ident: &Option<Ident>) -> Option<String> {
    let long = normalize_ident(ident.as_ref().unwrap());
    if long.len() > 1 { Some(long) } else { None }
}

fn field_short_is_valid(c: char) -> bool {
    !c.is_whitespace()
}

fn field_long_is_valid(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| !c.is_whitespace())
}

fn field_meta(ident: &Option<Ident>) -> String {
    normalize_ident(ident.as_ref().unwrap())
}

fn field_attrs(field: &Field, is_help: bool) -> Result<FieldAttrs, Error> {
    let ident = field.ident.as_ref().unwrap().clone();
    let arg_ident = format_ident!("arg_{}", ident);
    let (kind, outer_type, inner_type) = field_type(&field.ty)?;

    let mut field_attrs = FieldAttrs {
        ident,
        arg_ident,
        outer_type,
        inner_type,
        kind,
        short: field_short(&field.ident),
        long: field_long(&field.ident),
        meta: field_meta(&field.ident),
        variants: None,
        conflicts: vec![],
        choices: vec![],
        is_help,
        help0: String::new(),
        help1: None,
        help_error: String::new(),
    };

    for attr in &field.attrs {
        if attr.path().is_ident("cmdline") && matches!(attr.style, AttrStyle::Outer) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("short") {
                    let value = meta.value()?.parse::<LitChar>()?.value();
                    if !field_short_is_valid(value) {
                        return Err(meta.error("short cannot be whitespace"));
                    }
                    field_attrs.short = Some(value);
                } else if meta.path.is_ident("long") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if !field_long_is_valid(&value) {
                        return Err(meta.error("long cannot be empty or have whitespaces"));
                    }
                    field_attrs.long = Some(value);
                } else if meta.path.is_ident("no_short") {
                    if field_attrs.long.is_none() {
                        return Err(meta.error("cannot use both 'no_short' and 'no_long'"));
                    }
                    field_attrs.short = None;
                } else if meta.path.is_ident("no_long") {
                    if field_attrs.short.is_none() {
                        return Err(meta.error("cannot use both 'no_short' and 'no_long'"));
                    }
                    field_attrs.long = None;
                } else if meta.path.is_ident("positional") {
                    field_attrs.short = None;
                    field_attrs.long = None;
                } else if meta.path.is_ident("meta") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    field_attrs.meta = value;
                } else if meta.path.is_ident("variants") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    field_attrs.variants = Some(value);
                } else if meta.path.is_ident("conflict") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    field_attrs.conflicts.push(value);
                } else if meta.path.is_ident("choice") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    field_attrs.choices.push(value);
                } else if meta.path.is_ident("help") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    field_attrs.help1 = Some(value);
                } else {
                    return Err(meta.error("unsupported attribute for struct field"));
                }

                Ok(())
            })?;
        }
    }

    field_attrs.help0 = match (&field_attrs.short, &field_attrs.long, &field_attrs.kind) {
        (Some(short), None, FieldKind::OptionUnit) => format!("-{}", short),
        (None, Some(long), FieldKind::OptionUnit) => format!("--{}", long),
        (Some(short), Some(long), FieldKind::OptionUnit) => format!("-{}, --{}", short, long),
        (Some(short), None, _) => format!("-{} <{}>", short, &field_attrs.meta),
        (None, Some(long), _) => format!("--{} <{}>", long, &field_attrs.meta),
        (Some(short), Some(long), _) => format!("-{}, --{} <{}>", short, long, &field_attrs.meta),
        (None, None, FieldKind::OptionUnit) => String::new(),
        (None, None, FieldKind::OptionVec) => format!("[{}...]", &field_attrs.meta),
        (None, None, FieldKind::OptionOther) => format!("[{}]", &field_attrs.meta),
        (None, None, FieldKind::Vec) => format!("<{}...>", &field_attrs.meta),
        (None, None, FieldKind::Other) => format!("<{}>", &field_attrs.meta),
    };

    field_attrs.help_error = match (&field_attrs.short, &field_attrs.long, &field_attrs.kind) {
        (_, Some(long), FieldKind::OptionUnit) => format!("--{}", long),
        (_, Some(long), _) => format!("--{} <{}>", long, &field_attrs.meta),
        (Some(short), _, FieldKind::OptionUnit) => format!("-{}", short),
        (Some(short), _, _) => format!("-{} <{}>", short, &field_attrs.meta),
        (None, None, _) => field_attrs.help0.clone(),
    };

    Ok(field_attrs)
}

struct EnumAttrs {
    ident: Ident,
    meta: String,
    variants: Vec<VariantAttrs>,
}

fn enum_attrs(input: &DeriveInput, data: &DataEnum) -> Result<EnumAttrs, Error> {
    let mut enumeration = EnumAttrs {
        ident: input.ident.clone(),
        meta: enum_meta(&input.ident),
        variants: vec![],
    };

    for variant in &data.variants {
        enumeration.variants.push(variant_attrs(variant)?);
    }

    Ok(enumeration)
}

fn enum_meta(ident: &Ident) -> String {
    normalize_ident(ident)
}

enum VariantKind {
    Unit,
    Unnamed,
}

struct VariantAttrs {
    ident: Ident,
    kind: VariantKind,
    name: String,
    aliases: Vec<String>,
    help0: String,
    help1: Option<String>,
}

fn variant_kind(variant: &Variant) -> Result<VariantKind, Error> {
    match &variant.fields {
        Fields::Unit => Ok(VariantKind::Unit),
        Fields::Unnamed(fields) if fields.unnamed.len() == 1 => Ok(VariantKind::Unnamed),
        _ => Err(cannot_derive(
            variant.ident.span(),
            "enum variant (cannot have named fields)",
        )),
    }
}

fn variant_name(ident: &Ident) -> String {
    normalize_ident(ident)
}

fn variant_name_is_valid(name: &str) -> bool {
    !name.is_empty() && name.chars().all(|c| !c.is_whitespace())
}

fn variant_attrs(variant: &Variant) -> Result<VariantAttrs, Error> {
    let mut variant_attrs = VariantAttrs {
        ident: variant.ident.clone(),
        kind: variant_kind(&variant)?,
        name: variant_name(&variant.ident),
        aliases: vec![],
        help0: String::new(),
        help1: None,
    };

    for attr in &variant.attrs {
        if attr.path().is_ident("cmdline") && matches!(attr.style, AttrStyle::Outer) {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("name") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if !variant_name_is_valid(&value) {
                        return Err(meta.error(
                            "invalid name for enum variant (cannot be empty or have whitespaces)",
                        ));
                    }
                    variant_attrs.name = value;
                } else if meta.path.is_ident("alias") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if !variant_name_is_valid(&value) {
                        return Err(meta.error(
                            "invalid alias for enum variant (cannot be empty or have whitespaces)",
                        ));
                    }
                    variant_attrs.aliases.push(value);
                } else if meta.path.is_ident("help") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    variant_attrs.help1 = Some(value);
                } else {
                    return Err(meta.error("unsupported attribute for enum variant"));
                }

                Ok(())
            })?;
        }
    }

    variant_attrs.help0 = {
        let mut help0 = vec![variant_attrs.name.clone()];
        help0.extend_from_slice(&variant_attrs.aliases);
        help0.join(", ")
    };

    Ok(variant_attrs)
}

fn struct_help_usage(structure: &StructAttrs) -> TokenStream2 {
    let non_positional = structure
        .fields
        .iter()
        .filter(|field| !matches!((&field.short, &field.long), (None, None)))
        .collect::<Vec<_>>();
    let positional = structure
        .fields
        .iter()
        .filter(|field| matches!((&field.short, &field.long), (None, None)))
        .collect::<Vec<_>>();
    let help_non_positional = match non_positional.is_empty() {
        true => quote! {},
        false => quote! {
            let _ = write!(buf, " [options]");
        },
    };
    let help_positional = positional
        .iter()
        .map(|field| {
            let inner_type = &field.inner_type;
            let meta = &field.meta;
            let usage = format!(" {}", &field.help0);
            quote! {
                let _ = write!(buf, #usage);
                ::cmdline::help_usage::<#inner_type>(buf, #meta);
            }
        })
        .collect::<Vec<_>>();

    quote! {
        use ::std::fmt::Write;
        let _ = write!(buf, "usage: {}", meta);
        #help_non_positional
        #(#help_positional)*
    }
}

fn struct_help_options(structure: &StructAttrs) -> TokenStream2 {
    let non_positional = structure
        .fields
        .iter()
        .filter(|field| !matches!((&field.short, &field.long), (None, None)))
        .collect::<Vec<_>>();
    let help0_width = non_positional
        .iter()
        .map(|field| field.help0.len())
        .max()
        .unwrap_or_default();
    let help_options = non_positional
        .iter()
        .enumerate()
        .map(|(i, field)| {
            let help = match (&field.help0, &field.help1) {
                (help0, Some(help1)) => format!("   {:<help0_width$}     {}", help0, help1),
                (help0, None) => format!("   {}", help0),
            };
            let help = match i {
                0 => vec![String::from("\noptions:"), help],
                _ => vec![help],
            };
            quote! {
                #(let _ = writeln!(buf, #help);)*
            }
        })
        .collect::<Vec<_>>();
    let mut help_unique_variants = HashSet::new();
    let help_variants = structure
        .fields
        .iter()
        .filter(|field| match &field.variants {
            Some(variant) => help_unique_variants.insert(variant),
            None => false,
        })
        .map(|field| {
            let inner_type = &field.inner_type;
            let variants = &field.variants;
            quote! {
                ::cmdline::help_options::<#inner_type>(buf, #variants);
            }
        })
        .collect::<Vec<_>>();

    quote! {
        use ::std::fmt::Write;
        let _ = writeln!(buf);
        #(#help_options)*
        #(#help_variants)*
        let _ = writeln!(buf);
    }
}

fn enum_help_usage(enumeration: &EnumAttrs) -> TokenStream2 {
    match enumeration
        .variants
        .iter()
        .any(|variant| matches!(variant.kind, VariantKind::Unnamed))
    {
        true => quote! {
            use ::std::fmt::Write;
            let _ = write!(buf, " [{} options]", meta);
        },
        false => quote! {},
    }
}

fn enum_help_options(enumeration: &EnumAttrs) -> TokenStream2 {
    let help0_width = enumeration
        .variants
        .iter()
        .map(|variant| variant.help0.len())
        .max()
        .unwrap_or_default();
    let help_variants = enumeration
        .variants
        .iter()
        .map(|field| {
            let help = match (&field.help0, &field.help1) {
                (help0, Some(help1)) => format!("   {:<help0_width$}     {}", help0, help1),
                (help0, None) => format!("   {}", help0),
            };
            quote! {
                let _ = writeln!(buf, #help);
            }
        })
        .collect::<Vec<_>>();

    match help_variants.is_empty() {
        true => quote! {},
        false => quote! {
            use ::std::fmt::Write;
            let _ = writeln!(buf, "\n{}:", variants);
            #(#help_variants)*
        },
    }
}

fn normalize_ident(ident: &Ident) -> String {
    ident.to_string().to_lowercase().replace("_", "-")
}

fn cannot_derive<T: std::fmt::Display>(span: Span, message: T) -> Error {
    let message = format!("cannot derive CmdLine for {}", message);
    Error::new(span, message)
}
