#[macro_use]
extern crate clap;
extern crate chef;
extern crate chef_api;

use chef_api::api_client::*;
use chef::models::DataBagList;

use clap::{Arg, App, SubCommand};

fn main() {
    let command = App::new("chef")
        .bin_name("chef data bag list")
        .version(crate_version!())
        .arg(Arg::with_name("profile")
             .long("profile")
             .value_name("PROFILE")
             .help("Which set of credentials to use"))
        .get_matches();

    let profile = command.value_of("profile");
    let client = ApiClient::from_credentials(profile).unwrap();
    let bags: DataBagList = client.data().get().unwrap().into();
    for n in bags {
        println!("{}", n);
    }
}
