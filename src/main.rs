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
//extern crate iron;
extern crate router;
extern crate rustc_serialize;

use docopt::Docopt;
//use iron::prelude::*;
//use iron::status;
//use router::Router;

// local
mod init;
use init::pb_init;

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
    arg_username: Vec<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    println!("{:?}", args);

    if args.cmd_init {
        pb_init(args.arg_username);
    } else if args.cmd_run {
        unimplemented!();
    } else if args.cmd_list {
        unimplemented!();
    }

    //println!("PhotoBackup client listening on http://");
    //launch_server();
}
/*
fn launch_server() {

    // route handlers
    fn index_handler(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "redirect to http://PhotoBackup.github.io")))
    }

    fn post_handler(req: &mut Request) -> IronResult<Response> {
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
    Iron::new(router).http("localhost:3000").unwrap();
}
*/
