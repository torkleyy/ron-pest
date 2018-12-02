pub use self::parser::{RonParser, Rule};

/*
#[cfg(not(ide))]
mod parser {
    #[derive(Parser)]
    #[grammar = "grammar.pest"]
    pub struct RonParser;
}
*/

#[cfg(ide)]
mod parser {
    use pest::Parser;

    pub struct RonParser;

    #[allow(dead_code, non_camel_case_types)]
    #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub enum Rule { EOI, WHITESPACE, COMMENT, line_comment, line_comment_inner, block_comment, block_comment_inner, ident, ron_file, extension, value, bool, sign, signed_int, unsigned_int, with_base, float, float_std, float_frac, float_exp, char, char_inner, string, string_std, string_std_inner, string_raw, string_raw_inner, list, map, map_entry, unit_type, tuple_type, fields_type, field }

    impl Parser<Rule> for RonParser {
        fn parse<'i>(rule: Rule, input: &'i str) -> ::std::result::Result<::pest::iterators::Pairs<'i, Rule>, ::pest::error::Error<Rule>> {
            mod rules {
                pub mod hidden {
                    use super::super::Rule;

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn skip(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { if state.atomicity() == ::pest::Atomicity::NonAtomic { state.sequence(|state| { state.repeat(|state| { super::visible::WHITESPACE(state) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::visible::COMMENT(state).and_then(|state| { state.repeat(|state| { super::visible::WHITESPACE(state) }) }) }) }) }) }) } else { Ok(state) } }
                }

                pub mod visible {
                    use super::super::Rule;

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn WHITESPACE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::Atomic, |state| { state.match_string(" ").or_else(|state| { state.match_string("\n") }).or_else(|state| { state.match_string("\t") }).or_else(|state| { state.match_string("\r") }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn COMMENT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::CompoundAtomic, |state| { state.rule(Rule::COMMENT, |state| { self::line_comment(state).or_else(|state| { self::block_comment(state) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn line_comment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::line_comment, |state| { state.sequence(|state| { state.match_string("//").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::line_comment_inner(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::NEWLINE(state).or_else(|state| { state.lookahead(true, |state| { self::EOI(state) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn line_comment_inner(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.lookahead(false, |state| { self::NEWLINE(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.lookahead(false, |state| { self::NEWLINE(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn block_comment(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::block_comment, |state| { state.sequence(|state| { state.match_string("/*").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::block_comment(state).or_else(|state| { self::block_comment_inner(state) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("*/") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn block_comment_inner(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("*/") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("*/") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn ident(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ident, |state| { state.atomic(::pest::Atomicity::Atomic, |state| { state.sequence(|state| { self::ASCII_ALPHA(state).or_else(|state| { state.match_string("_") }).and_then(|state| { state.repeat(|state| { self::ASCII_ALPHANUMERIC(state).or_else(|state| { state.match_string("_") }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn ron_file(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::ron_file, |state| { state.sequence(|state| { self::SOI(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::extension(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::extension(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::EOI(state) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn extension(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::extension, |state| { state.sequence(|state| { state.match_string("#").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("!") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("[") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("enable") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ident(state) }) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string(",") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("]") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn value(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::value, |state| { self::float(state).or_else(|state| { self::signed_int(state) }).or_else(|state| { self::bool(state) }).or_else(|state| { self::char(state) }).or_else(|state| { state.restore_on_err(|state| self::string(state)) }).or_else(|state| { state.restore_on_err(|state| self::list(state)) }).or_else(|state| { state.restore_on_err(|state| self::map(state)) }).or_else(|state| { state.restore_on_err(|state| self::tuple_type(state)) }).or_else(|state| { state.restore_on_err(|state| self::fields_type(state)) }).or_else(|state| { self::unit_type(state) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn bool(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::bool, |state| { state.match_string("true").or_else(|state| { state.match_string("false") }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn sign(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::sign, |state| { state.match_string("+").or_else(|state| { state.match_string("-") }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn signed_int(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::signed_int, |state| { state.atomic(::pest::Atomicity::Atomic, |state| { state.sequence(|state| { state.optional(|state| { self::sign(state) }).and_then(|state| { self::unsigned_int(state) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn unsigned_int(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::unsigned_int, |state| { self::with_base(state).or_else(|state| { state.sequence(|state| { self::ASCII_DIGIT(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_DIGIT(state) }) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn with_base(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::with_base, |state| { state.sequence(|state| { state.match_string("0").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("x").or_else(|state| { state.match_string("b") }).or_else(|state| { state.match_string("o") }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ASCII_HEX_DIGIT(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_HEX_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_HEX_DIGIT(state) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn float(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::float, |state| { state.atomic(::pest::Atomicity::Atomic, |state| { self::float_std(state).or_else(|state| { self::float_frac(state) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn float_std(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::float_std, |state| { state.sequence(|state| { state.optional(|state| { self::sign(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { self::ASCII_DIGIT(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_DIGIT(state) }) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(".") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_DIGIT(state) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::float_exp(state) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn float_frac(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::float_frac, |state| { state.sequence(|state| { state.match_string(".").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { self::ASCII_DIGIT(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_DIGIT(state) }) }) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { self::float_exp(state) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn float_exp(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::float_exp, |state| { state.sequence(|state| { state.match_string("e").or_else(|state| { state.match_string("E") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ASCII_DIGIT(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { self::ASCII_DIGIT(state).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { self::ASCII_DIGIT(state) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn char(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::CompoundAtomic, |state| { state.rule(Rule::char, |state| { state.sequence(|state| { state.match_string("\'").and_then(|state| { self::char_inner(state) }).and_then(|state| { state.match_string("\'") }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn char_inner(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::char_inner, |state| { state.match_string("\\\'").or_else(|state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("\'") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("\'") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.atomic(::pest::Atomicity::CompoundAtomic, |state| { state.rule(Rule::string, |state| { self::string_std(state).or_else(|state| { state.restore_on_err(|state| self::string_raw(state)) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_std(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::string_std, |state| { state.sequence(|state| { state.match_string("\"").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::string_std_inner(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("\"") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_std_inner(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::string_std_inner, |state| { state.sequence(|state| { state.optional(|state| { state.match_string("\\\"").or_else(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("\"") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.match_string("\\\"").or_else(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.match_string("\"") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_raw(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::string_raw, |state| { state.sequence(|state| { state.match_string("r").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.stack_push(|state| state.sequence(|state| { state.optional(|state| { state.match_string("#").and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.match_string("#") }) }) }) }) }) })) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("\"") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::string_raw_inner(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("\"") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::POP(state) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn string_raw_inner(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::string_raw_inner, |state| { state.sequence(|state| { state.optional(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.sequence(|state| { state.match_string("\"").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::PEEK(state) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.sequence(|state| { state.lookahead(false, |state| { state.sequence(|state| { state.match_string("\"").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::PEEK(state) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::ANY(state) }) }) }) }) }) }) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn list(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::list, |state| { state.sequence(|state| { state.match_string("[").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { self::value(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) })).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) })) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string(",") }) }) })) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("]") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn map(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::map, |state| { state.sequence(|state| { state.match_string("{").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { self::map_entry(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::map_entry(state) }) })).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::map_entry(state) }) })) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string(",") }) }) })) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("}") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn map_entry(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::map_entry, |state| { state.sequence(|state| { self::value(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn unit_type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::unit_type, |state| { self::ident(state) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn tuple_type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::tuple_type, |state| { state.sequence(|state| { state.optional(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { self::value(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) })).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) })) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string(",") }) }) })) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn fields_type(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::fields_type, |state| { state.sequence(|state| { state.optional(|state| { self::ident(state) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string("(") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { self::field(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.sequence(|state| { state.optional(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field(state) }) })).and_then(|state| { state.repeat(|state| { state.sequence(|state| { super::hidden::skip(state).and_then(|state| { state.restore_on_err(|state| state.sequence(|state| { state.match_string(",").and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::field(state) }) })) }) }) }) }) }) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.optional(|state| { state.match_string(",") }) }) })) }) }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(")") }) }) }) }

                    #[inline]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn field(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::field, |state| { state.sequence(|state| { self::ident(state).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { state.match_string(":") }).and_then(|state| { super::hidden::skip(state) }).and_then(|state| { self::value(state) }) }) }) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn POP(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.stack_pop() }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_HEX_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_range('0'..'9').or_else(|state| state.match_range('a'..'f')).or_else(|state| state.match_range('A'..'F')) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn PEEK(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.stack_peek() }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn EOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.rule(Rule::EOI, |state| state.end_of_input()) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_ALPHANUMERIC(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z')).or_else(|state| state.match_range('0'..'9')) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn SOI(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.start_of_input() }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn NEWLINE(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_string("\n").or_else(|state| state.match_string("\r\n")).or_else(|state| state.match_string("\r")) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_DIGIT(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_range('0'..'9') }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ASCII_ALPHA(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.match_range('a'..'z').or_else(|state| state.match_range('A'..'Z')) }

                    #[inline]
                    #[allow(dead_code, non_snake_case, unused_variables)]
                    pub fn ANY(state: Box<::pest::ParserState<Rule>>) -> ::pest::ParseResult<Box<::pest::ParserState<Rule>>> { state.skip(1) }
                }

                pub use self::visible::*;
            } ::pest::state(input, |state| {
                match rule {
                    Rule::WHITESPACE => rules::WHITESPACE(state),
                    Rule::COMMENT => rules::COMMENT(state),
                    Rule::line_comment => rules::line_comment(state),
                    Rule::line_comment_inner => rules::line_comment_inner(state),
                    Rule::block_comment => rules::block_comment(state),
                    Rule::block_comment_inner => rules::block_comment_inner(state),
                    Rule::ident => rules::ident(state),
                    Rule::ron_file => rules::ron_file(state),
                    Rule::extension => rules::extension(state),
                    Rule::value => rules::value(state),
                    Rule::bool => rules::bool(state),
                    Rule::sign => rules::sign(state),
                    Rule::signed_int => rules::signed_int(state),
                    Rule::unsigned_int => rules::unsigned_int(state),
                    Rule::with_base => rules::with_base(state),
                    Rule::float => rules::float(state),
                    Rule::float_std => rules::float_std(state),
                    Rule::float_frac => rules::float_frac(state),
                    Rule::float_exp => rules::float_exp(state),
                    Rule::char => rules::char(state),
                    Rule::char_inner => rules::char_inner(state),
                    Rule::string => rules::string(state),
                    Rule::string_std => rules::string_std(state),
                    Rule::string_std_inner => rules::string_std_inner(state),
                    Rule::string_raw => rules::string_raw(state),
                    Rule::string_raw_inner => rules::string_raw_inner(state),
                    Rule::list => rules::list(state),
                    Rule::map => rules::map(state),
                    Rule::map_entry => rules::map_entry(state),
                    Rule::unit_type => rules::unit_type(state),
                    Rule::tuple_type => rules::tuple_type(state),
                    Rule::fields_type => rules::fields_type(state),
                    Rule::field => rules::field(state),
                    Rule::EOI => rules::EOI(state)
                }
            })
        }
    }
}
