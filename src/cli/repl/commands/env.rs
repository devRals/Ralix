use super::Repl;
pub(super) fn run(repl: &mut Repl, arguments: &[String]) {
    if arguments.is_empty() {
        return;
    }

    let cmd = &**arguments.first().unwrap();

    if cmd == "clear" {
        repl.context.env.clear();
        repl.context.symbol_table.clear();
    };
}

pub(super) fn content(arguments: &[String]) -> &'static str {
    if arguments.is_empty() {
        return r#"Manipulate the environment using commands
Available Commands:
    clear: "Removes the all of the environment variables"
    ... Umm... yeah thats all... for now :3"#;
    }

    let cmd = &**arguments.first().unwrap();
    match cmd {
        "clear" => "Removes the all defined variables",
        _ => "Subcommand not found",
    }
}
