/*
 * Copyright (C) 2013-2016 Stéphane Péchard.
 *
 * This file is part of PhotoBackup.
 *
 * PhotoBackup is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * PhotoBackup is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

extern crate docopt;
extern crate ini;
extern crate iron;
//extern crate params;
extern crate router;
extern crate rustc_serialize;

use docopt::Docopt;
use iron::modifiers::Redirect;
use ini::Ini;
use iron::prelude::*;
use iron::{status, Url};
//use params::{Params, Value};
use router::Router;
//use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

// local
mod init;
use init::{get_suffix, pb_init};


// command line documentation
const USAGE: &'static str = "
PhotoBackup Rust server.

Usage:
  photobackup init [<username>]
  photobackup run [<username>]
  photobackup list
  photobackup (-h | --help)
  photobackup --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";


#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_init: bool,
    cmd_run: bool,
    cmd_list: bool,
    arg_username: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    let pb_path = get_path();
    read_conf(&args.arg_username, &pb_path);

    if args.cmd_init {
        pb_init(args.arg_username, pb_path);
    } else if args.cmd_run {
        launch_server(args.arg_username, pb_path);
    } else if args.cmd_list {
        unimplemented!();
    }

}


fn get_path() -> String {
    let home_dir = env::home_dir().unwrap();
    let mut path = PathBuf::from(home_dir);
    path.push(".photobackup");
    return path.to_string_lossy().into_owned();
}


fn read_conf(username: &String, path: &String) { //-> HashMap<String, String> {
    let conf = Ini::load_from_file(path).unwrap();
    let section_title = "photobackup".to_string() + &get_suffix(&username);
    let section = conf.section(Some(section_title)).unwrap();
    //return section;
}


fn launch_server(username: String, path: String) {

    // route handlers
    fn index_handler(_req: &mut Request) -> IronResult<Response> {
        let url = Url::parse("http://PhotoBackup.github.io").unwrap();
        Ok(Response::with((status::Found, Redirect(url.clone()))))
    }

    fn post_handler(req: &mut Request) -> IronResult<Response> {
        //validate_password(req);
        Ok(Response::with((status::Ok, "post image => save it!")))
    }

    fn test_handler(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "test it!")))
    }

    // router
    let mut router = Router::new();
    router.get("/", index_handler);
    router.post("/", post_handler);
    router.post("/test", test_handler);

    // finally run it
    println!("PhotoBackup client listening on http://");
    Iron::new(router).http("127.0.0.1:3000").unwrap();
}


/*fn validate_password(req: &mut Request) {
    let map = req.get_ref::<Params>().unwrap();
    match map.find(&["password"]) {
        Some(&Value::String(ref password)) if password == "Marie" => {
            Ok(Response::with((iron::status::Ok, "Welcome back, Marie!")))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}
*/
