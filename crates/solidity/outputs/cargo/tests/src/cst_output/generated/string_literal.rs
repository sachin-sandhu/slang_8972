// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn escape_cr_double_quote() -> Result<()> {
    run("StringLiteral", "escape_cr_double_quote")
}

#[test]
fn escape_cr_single_quote() -> Result<()> {
    run("StringLiteral", "escape_cr_single_quote")
}

#[test]
fn escape_crlf_double_quote() -> Result<()> {
    run("StringLiteral", "escape_crlf_double_quote")
}

#[test]
fn escape_crlf_single_quote() -> Result<()> {
    run("StringLiteral", "escape_crlf_single_quote")
}

#[test]
fn escape_lf_double_quote() -> Result<()> {
    run("StringLiteral", "escape_lf_double_quote")
}

#[test]
fn escape_lf_single_quote() -> Result<()> {
    run("StringLiteral", "escape_lf_single_quote")
}

#[test]
fn tabs_double_quote() -> Result<()> {
    run("StringLiteral", "tabs_double_quote")
}

#[test]
fn tabs_single_quote() -> Result<()> {
    run("StringLiteral", "tabs_single_quote")
}
