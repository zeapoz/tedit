use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, parse_macro_input, Ident, LitStr, Token, Type};

struct Command {
    name: Ident,
    description: LitStr,
    args: Punctuated<Arg, Token![,]>,
    handler: syn::Expr,
}

struct Arg {
    name: Ident,
    ty: Type,
}

struct Commands {
    commands: Punctuated<Command, Token![,]>,
}

// Parsing each argument in args: [name: type, ...].
impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Arg { name, ty })
    }
}

// Parsing one command block.
impl Parse for Command {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut description: Option<LitStr> = None;
        let mut args: Option<Punctuated<Arg, Token![,]>> = None;
        let mut handler: Option<syn::Expr> = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "description" => {
                    description = Some(content.parse()?);
                }
                "args" => {
                    let inner;
                    syn::bracketed!(inner in content);
                    let args_list = inner.parse_terminated(Arg::parse, Token![,])?;
                    args = Some(args_list);
                }
                "handler" => {
                    handler = Some(content.parse()?);
                }
                _ => return Err(syn::Error::new(key.span(), "Unknown field in command")),
            }

            let _ = content.parse::<Token![,]>();
        }

        Ok(Command {
            name: name.clone(),
            description: description
                .ok_or_else(|| syn::Error::new(name.span(), "Missing description"))?,
            args: args.unwrap_or_default(),
            handler: handler.ok_or_else(|| syn::Error::new(name.span(), "Missing handler"))?,
        })
    }
}

// Parsing multiple commands.
impl Parse for Commands {
    fn parse(input: ParseStream) -> Result<Self> {
        let commands = Punctuated::parse_terminated(input)?;
        Ok(Commands { commands })
    }
}

#[proc_macro]
pub fn define_commands(input: TokenStream) -> TokenStream {
    let Commands { commands } = parse_macro_input!(input as Commands);

    let mut command_structs = Vec::new();
    let mut command_specs = Vec::new();
    let mut registry_inserts = Vec::new();

    for cmd in commands {
        let cmd_name = &cmd.name;
        let cmd_spec = format_ident!("{}Spec", cmd_name);
        let description = &cmd.description;

        let arg_names: Vec<_> = cmd.args.iter().map(|a| &a.name).collect();
        let arg_types: Vec<_> = cmd.args.iter().map(|a| &a.ty).collect();

        let struct_fields = if cmd.args.is_empty() {
            quote! {}
        } else {
            quote! {
                #(
                    pub #arg_names: #arg_types,
                )*
            }
        };

        let arg_parse = if cmd.args.is_empty() {
            quote! {}
        } else {
            let mut parse_tokens = Vec::new();
            for arg in &cmd.args {
                let name = &arg.name;
                let ty = &arg.ty;

                // Check if type is Option<T>.
                let is_option = match ty {
                    syn::Type::Path(type_path) => type_path
                        .path
                        .segments
                        .last()
                        .map(|s| s.ident == "Option")
                        .unwrap_or(false),
                    _ => false,
                };

                if is_option {
                    parse_tokens.push(quote! {
                        let #name: #ty = iter.next().map(|v| v.parse()).transpose().unwrap_or(None);
                    });
                } else {
                    parse_tokens.push(quote! {
                        let #name: #ty = iter
                            .next()
                            .ok_or_else(|| crate::editor::command::Error::MissingArgument(stringify!(#name).to_string()))?
                            .parse::<#ty>()
                            .map_err(|e| crate::editor::command::Error::InvalidArgument {
                                name: stringify!(#name).to_string(),
                                error: e.to_string(),
                            })?;
                    });
                }
            }
            quote! { #( #parse_tokens )* }
        };

        let handler = &cmd.handler;

        // Command struct + `Command` impl.
        command_structs.push(quote! {
            pub struct #cmd_name {
                #struct_fields
            }

            impl #cmd_name {
                fn cmd_handler(&self, editor: &mut crate::editor::Editor) -> Result<(), crate::editor::command::Error> {
                    #handler
                    Ok(())
                }
            }

            impl crate::editor::command::Command for #cmd_name {
                fn execute(&self, editor: &mut crate::editor::Editor) -> Result<(), crate::editor::command::Error> {
                    self.cmd_handler(editor)
                }
            }
        });

        // Spec struct + `CommandSpec` impl.
        command_specs.push(quote! {
            pub struct #cmd_spec;

            impl crate::editor::command::CommandSpec for #cmd_spec {
                fn name(&self) -> &'static str {
                    static NAME: std::sync::OnceLock<String> = ::std::sync::OnceLock::new();
                    NAME.get_or_init(|| stringify!(#cmd_name).to_string())
                }

                fn description(&self) -> &'static str {
                    #description
                }

                fn parse(&self, raw_args: &str) -> Result<Box<dyn crate::editor::command::Command>, crate::editor::command::Error> {
                    let mut iter = raw_args.split_whitespace();
                    #arg_parse
                    Ok(Box::new(#cmd_name { #( #arg_names ),* }))
                }
            }
        });

        registry_inserts.push(cmd_spec);
    }

    let expanded = quote! {
        #(#command_structs)*
        #(#command_specs)*

        pub fn register_commands(registry: &mut crate::editor::command::CommandRegistry) {
            #(
                registry.register(std::rc::Rc::new(#registry_inserts {}));
            )*
        }
    };

    TokenStream::from(expanded)
}
