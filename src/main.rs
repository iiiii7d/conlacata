#![warn(
    clippy::as_underscore,
    clippy::bool_to_int_with_if,
    clippy::case_sensitive_file_extension_comparisons,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::checked_conversions,
    clippy::clone_on_ref_ptr,
    clippy::cloned_instead_of_copied,
    clippy::cognitive_complexity,
    clippy::copy_iterator,
    clippy::create_dir,
    clippy::default_trait_access,
    clippy::deref_by_slicing,
    clippy::doc_link_with_quotes,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::empty_line_after_outer_attr,
    clippy::empty_structs_with_brackets,
    clippy::enum_glob_use,
    clippy::equatable_if_let,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::explicit_iter_loop,
    clippy::filetype_is_file,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::fn_to_numeric_cast_any,
    clippy::from_iter_instead_of_collect,
    clippy::future_not_send,
    clippy::get_unwrap,
    clippy::if_not_else,
    clippy::if_then_some_else_none,
    clippy::implicit_hasher,
    clippy::imprecise_flops,
    clippy::inconsistent_struct_constructor,
    clippy::index_refutable_slice,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::items_after_statements,
    clippy::iter_not_returning_iterator,
    clippy::iter_on_empty_collections,
    clippy::iter_on_single_items,
    clippy::iter_with_drain,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::manual_assert,
    clippy::manual_instant_elapsed,
    clippy::manual_let_else,
    clippy::manual_ok_or,
    clippy::manual_string_new,
    clippy::many_single_char_names,
    clippy::map_err_ignore,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::mismatching_type_param_order,
    clippy::missing_const_for_fn,
    clippy::missing_enforced_import_renames,
    clippy::must_use_candidate,
    clippy::mut_mut,
    clippy::naive_bytecount,
    clippy::needless_bitwise_bool,
    clippy::needless_collect,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::negative_feature_names,
    clippy::non_ascii_literal,
    clippy::non_send_fields_in_send_ty,
    clippy::or_fun_call,
    clippy::range_minus_one,
    clippy::range_plus_one,
    clippy::rc_buffer,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::redundant_feature_names,
    clippy::redundant_pub_crate,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::return_self_not_must_use,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::separated_literal_suffix,
    clippy::significant_drop_in_scrutinee,
    clippy::single_match_else,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::string_slice,
    clippy::struct_excessive_bools,
    clippy::suboptimal_flops,
    clippy::suspicious_operation_groupings,
    clippy::suspicious_xor_used_as_pow,
    clippy::trailing_empty_array,
    clippy::trait_duplication_in_bounds,
    clippy::transmute_ptr_to_ptr,
    clippy::transmute_undefined_repr,
    clippy::trivial_regex,
    clippy::trivially_copy_pass_by_ref,
    clippy::try_err,
    clippy::type_repetition_in_bounds,
    clippy::undocumented_unsafe_blocks,
    clippy::unicode_not_nfc,
    clippy::uninlined_format_args,
    clippy::unnecessary_join,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unsafe_derive_deserialize,
    clippy::unused_async,
    clippy::unused_peekable,
    clippy::unused_rounding,
    clippy::unused_self,
    clippy::unwrap_in_result,
    clippy::use_self,
    clippy::useless_let_if_seq,
    clippy::verbose_bit_mask,
    clippy::verbose_file_reads
)]
#![deny(
    clippy::derive_partial_eq_without_eq,
    clippy::match_bool,
    clippy::mem_forget,
    clippy::mutex_atomic,
    clippy::mutex_integer,
    clippy::nonstandard_macro_braces,
    clippy::path_buf_push_overwrite,
    clippy::rc_mutex,
    clippy::wildcard_dependencies
)]

use std::path::PathBuf;

use ansi_term::Color::Red;
use clap::Parser;

use crate::types::ResultAnyError;

mod modules;
mod types;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The language folder to operate on
    lang_folder: PathBuf,
    #[clap(subcommand)]
    subcmd: Subcmd,
    /// Enables debugging info
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
}

#[derive(Parser)]
enum Subcmd {
    /// View the language's orthography
    #[clap(alias = "ortho")]
    Orthography(modules::orthography::OrthographyOptions),
    /// Translate a sentence in a conlang to IPA
    Ipa(modules::ipa::IpaOptions),
    /// View the language's parts of speech
    #[clap(alias = "pos")]
    PartsOfSpeech(modules::parts_of_speech::PartsOfSpeechOptions),
    /// View the language's lexicon
    #[clap(alias = "lex")]
    Lexicon(modules::lexicon::LexiconOptions),
    /// Conjugate a word in the language
    #[clap(alias = "conj")]
    Conjugation(modules::lexicon::ConjugationOptions),
}

pub trait CliOptions {
    fn run(&self, lang_folder: PathBuf) -> ResultAnyError<()>;
}

fn main() {
    let args = Args::parse();
    match args.subcmd {
        Subcmd::Orthography(v) => v.run(args.lang_folder),
        Subcmd::Ipa(v) => v.run(args.lang_folder),
        Subcmd::PartsOfSpeech(v) => v.run(args.lang_folder),
        Subcmd::Lexicon(v) => v.run(args.lang_folder),
        Subcmd::Conjugation(v) => v.run(args.lang_folder),
    }
    .unwrap_or_else(|e| {
        eprintln!("{}", Red.paint(e.to_string()));
    });
}
