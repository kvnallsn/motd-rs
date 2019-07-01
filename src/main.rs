//! MOTD Utility
//!
//! Parses motd templates (*.motd) and displays them to the console

mod commands;
mod error;
mod motd;

fn main() {
    //let template = read_template("templates/falcon.motd").expect("failed to open file");
    //let template = process_template(template);
    let motd = motd::Motd::new();
    let template = motd.render("templates/falcon.motd").unwrap();

    println!("{}", template);
}
