use syn::*;
use syn::parse::*;
use proc_macro2::*;
use proc_macro::TokenStream;

pub fn debug_impl(input: TokenStream) -> TokenStream {
	let output_token_stream_str = "fn foo() -> u32 { 42 }";
  let output = output_token_stream_str.parse().unwrap();

  let ast_item_fn: ItemFn = parse_str::<ItemFn>(output_token_stream_str).unwrap();
  visualize_ast(ast_item_fn);

  output
}

fn visualize_ast(ast: ItemFn) {
  // Simply dump the AST to the console.
  let ast_clone = ast.clone();
  eprintln!("{} => {}", style_primary("Debug::ast"), ast_clone);

  // Parse AST to dump some items to the console.
  let ItemFn {
    attrs,
    vis,
    sig,
    block,
  } = ast;

  eprintln!(
    "{} ast_item_fn < attrs.len:{}, vis:{}, sig:'{}' stmt: '{}' >",
    style_primary("=>"),
    style_prompt(&attrs.len().to_string()),
    style_prompt(match vis {
      syn::Visibility::Public(_) => "public",
      syn::Visibility::Crate(_) => "crate",
      syn::Visibility::Restricted(_) => "restricted",
      syn::Visibility::Inherited => "inherited",
    }),
    style_prompt(&sig.ident.to_string()),
    style_prompt(&match block.stmts.first() {
      Some(stmt) => {
        let expr_str = stmt.to_token_stream().to_string().clone();
        expr_str
      }
      None => "empty".to_string(),
    }),
  );
}