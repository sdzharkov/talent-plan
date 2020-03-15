extern crate clap;
extern crate kvs;
use clap::{App, Arg, SubCommand, AppSettings};

fn main() {
  let matches = App::new(env!("CARGO_PKG_NAME"))
                  .version(env!("CARGO_PKG_VERSION"))
                  .author(env!("CARGO_PKG_AUTHORS"))
                  .about(env!("CARGO_PKG_DESCRIPTION"))
                  .setting(AppSettings::ArgRequiredElseHelp)
                  .subcommands(vec![
                    SubCommand::with_name("get")
                      .about("Get the value")
                      .arg(
                        Arg::with_name("key")
                          .takes_value(true)
                          .value_name("KEY")
                          .required(true)
                          .index(1)
                      ),
                    SubCommand::with_name("set")
                      .about("set the value")
                      .arg(
                        Arg::with_name("key")
                          .takes_value(true)
                          .value_name("KEY")
                          .required(true)
                          .index(1)
                      )
                      .arg(
                        Arg::with_name("value")
                          .takes_value(true)
                          .value_name("VALUE")
                          .required(true)
                          .index(2)
                      ),
                    SubCommand::with_name("rm")
                      .about("Remove a value")
                      .arg(
                        Arg::with_name("key")
                          .takes_value(true)
                          .value_name("KEY")
                          .required(true)
                          .index(1)
                      ),
                  ])
                  .get_matches();

  
  match matches.subcommand() {
    ("get", Some(get_matches)) => {
      let key = get_matches.value_of("key").unwrap();
      panic!("unimplemented");
    }
    ("set", Some(set_matches)) => {
      panic!("unimplemented");
    }
    ("rm", Some(rm_matches)) => {
      panic!("unimplemented");
    }
    ("", None) => println!("No subcommand was used"), // If no subcommand was usd it'll match the tuple ("", None)
    _ => unreachable!(),
  }
}
