use colored::*;

pub fn print_intro() {
    println!(
        "{}",
        format!(
            "{} - {}\n{}\nUse .help to list metacommands.\nFor more information on how it works, refer to /util/Schemas/schema.sql.",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            "Developed by Huzaifa Naseer."
        )
        .blue()
        .bold()
    );
}
