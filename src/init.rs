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

extern crate bcrypt;
extern crate crypto;
extern crate ini;
extern crate rpassword;

use self::bcrypt::{DEFAULT_COST, hash};
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512;
use self::ini::Ini;
use self::rpassword::read_password;
use std::io::{self, Write};


pub fn pb_init(username: String, path: String) {
    // mediaroot
    print!("The directory where to put the pictures (should be writable by the server you use): ");
    io::stdout().flush().unwrap();
    let mut mediaroot = String::new();
    io::stdin().read_line(&mut mediaroot)
        .expect("ERROR: Failed to read the line you entered!");
    // trim() because we get the carriage return in the string...
    mediaroot = mediaroot.trim().to_string();

    // password
    print!("The server password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();

    // create sha512 hash and Bcrypt crypted hash
    let mut hasher = Sha512::new();
    hasher.input_str(&*password);
    let pass_sha = hasher.result_str();
    // limit pass sha length to 72 characters (bcrypt limitation)
    let limited_pass_sha = &pass_sha[0..71];

    let passhash = match hash(&*limited_pass_sha, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!()
    };

    // create ini file
    let section = "photobackup".to_string() + &get_suffix(&username);

    let mut conf = Ini::new();
    conf.with_section(Some(section.to_owned()))
        .set("BindAddress", "127.0.0.1")
        .set("MediaRoot", mediaroot)
        .set("Password", pass_sha.to_owned())
        .set("PasswordBcrypt", passhash)
        .set("Port", "8420");
    conf.write_to_file(&path).unwrap();
}


pub fn get_suffix(username: &String) -> String {
    let mut suffix = "".to_string(); // default
    if !username.is_empty() {
        suffix = "-".to_string() + username;
    }
    return suffix;
}
