use error::*;

use clap::{self, Arg, App};

#[derive(Debug)]
pub struct Args {
    pub input_name: String,
}

impl Args {
    pub fn parse() -> Result<Args> {
        let args =
            App::new(crate_name!())
                .version(crate_authors!())
                .author(crate_version!())
                .about("")
                .arg(Arg::with_name("INPUT")
                     .help("input file to open")
                     .required(true)
                     .index(1))
                .get_matches();

        ArgMatches(args).to_args()
    }
}

struct ArgMatches<'a>(clap::ArgMatches<'a>);

impl<'a> ::std::ops::Deref for ArgMatches<'a> {
    type Target = clap::ArgMatches<'a>;

    fn deref(&self) -> &clap::ArgMatches<'a> { &self.0 }
}

impl<'a> ArgMatches<'a> {
    fn to_args(&self) -> Result<Args> {
        Ok(Args {
            input_name: self.input_name(),
        })
    }

    fn input_name(&self) -> String {
        self.value_of("INPUT").unwrap().into()
    }
}
