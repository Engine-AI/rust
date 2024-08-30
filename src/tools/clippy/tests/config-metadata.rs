#![feature(rustc_private)]

use clippy_config::{get_configuration_metadata, ClippyConfiguration};
use itertools::Itertools;
use regex::Regex;
use std::borrow::Cow;
use std::{env, fs};

fn metadata() -> impl Iterator<Item = ClippyConfiguration> {
    get_configuration_metadata()
        .into_iter()
        .filter(|config| config.deprecation_reason.is_none())
        .filter(|config| !config.lints.is_empty())
}

#[test]
fn book() {
    let path = "book/src/lint_configuration.md";
    let current = fs::read_to_string(path).unwrap();

    let configs = metadata().map(|conf| conf.to_markdown_paragraph()).join("\n");
    let expected = format!(
        r#"<!--
This file is generated by `cargo bless --test config-metadata`.
Please use that command to update the file and do not edit it by hand.
-->

# Lint Configuration Options

The following list shows each configuration option, along with a description, its default value, an example
and lints affected.

---

{}
"#,
        configs.trim(),
    );

    if current != expected {
        if env::var_os("RUSTC_BLESS").is_some_and(|v| v != "0") {
            fs::write(path, expected).unwrap();
        } else {
            panic!("`{path}` is out of date, run `cargo bless --test config-metadata` to update it");
        }
    }
}

#[test]
fn changelog() {
    let path = "CHANGELOG.md";
    let current = fs::read_to_string(path).unwrap();

    let configs = metadata().map(|conf| conf.to_markdown_link()).join("\n");

    let re = Regex::new(
        "(?s)\
        (<!-- begin autogenerated links to configuration documentation -->)\
        .*\
        (<!-- end autogenerated links to configuration documentation -->)\
        ",
    )
    .unwrap();
    let expected = re.replace(&current, format!("$1\n{configs}\n$2"));

    assert!(
        matches!(expected, Cow::Owned(_)),
        "failed to find configuration section in `{path}`"
    );

    if current != expected {
        if env::var_os("RUSTC_BLESS").is_some_and(|v| v != "0") {
            fs::write(path, expected.as_bytes()).unwrap();
        } else {
            panic!("`{path}` is out of date, run `cargo bless --test config-metadata` to update it");
        }
    }
}
