pub(super) fn content(arguments: &[String]) -> &'static str {
    let _default = r#"Documents about the interpreter and the commands:
Available Subcommands are: 
    "variables", "functions", "types", "environment"
"#;
    if arguments.is_empty() {
        return _default;
    }

    let subcmd = &**arguments.first().unwrap();
    match subcmd {
        "variables" => include_str!("./helpdocs/variables.md"),
        "functions" => include_str!("./helpdocs/functions.md"),
        "types" => include_str!("./helpdocs/types.md"),
        "environment" => include_str!("./helpdocs/environment.md"),
        _ => _default,
    }
}
