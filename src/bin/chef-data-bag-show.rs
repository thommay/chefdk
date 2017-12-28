#[macro_use]
extern crate clap;
extern crate chef;
extern crate chef_api;
extern crate serde_json;

use chef_api::api_client::*;
use chef::models::DataBagItemList;
use clap::{Arg, App};

fn main() {
    let command = App::new("chef")
        .bin_name("chef data bag show")
        .version(crate_version!())
        .arg(Arg::with_name("profile")
             .long("profile")
             .value_name("PROFILE")
             .help("Which set of credentials to use"))
        .arg(Arg::with_name("bag").required(true))
        .arg(Arg::with_name("item"))
        .get_matches();

    let profile = command.value_of("profile");
    let client = ApiClient::from_credentials(profile).unwrap();
    let bag = command.value_of("bag").unwrap();
    if let Some(item) = command.value_of("item") {
        let data = client.data().data_bag(bag).item(item).get().unwrap();
        println!("{}", serde_json::to_string_pretty(&data).unwrap());
    } else {
        let bag: DataBagItemList = client.data().data_bag(bag).get().unwrap().into();
        for n in bag {
            println!("{}", n);
        }
    }
}
