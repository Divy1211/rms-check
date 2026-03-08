use std::fs;
use std::path::PathBuf;
use chumsky::prelude::*;
use rmsc_core::{lexer, parser};

fn main() {
    let path = r"C:\Program Files (x86)\Steam\steamapps\common\AoE2DE\resources\_common\drs\gamedata_x2\Arabia.rms";
    let path = PathBuf::from(path);

    // let src = "#const 2_p_game";
    let src = fs::read_to_string(path).unwrap();

    let (tokens, errs) = lexer()
        .parse(&src)
        .into_output_errors();

    // println!("{:#?} {:#?}", tokens, errs);

    let tokens = tokens.unwrap();

    let (tokens, comments) = tokens.into_iter()
        .partition::<Vec<_>, _>(|tok| !tok.0.is_comment());

    // let comments = comments.into_iter()
    //     .map(|(val, span)| match val {
    //         Token::Comment(msg) => (msg, span),
    //         _ => unreachable!(),
    //     }).collect();

    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    println!("{:#?} {:#?}", ast, errs);
}
