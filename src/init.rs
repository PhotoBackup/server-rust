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

extern crate rpassword;

use self::rpassword::read_password;
use std::io::{self, Write};


pub fn pb_init(username: Vec<String>) {
    // mediaroot
    print!("The directory where to put the pictures (should be writable by the server you use): ");
    io::stdout().flush().unwrap();
    let mut mediaroot = String::new();
    io::stdin().read_line(&mut mediaroot)
        .expect("ERROR: Failed to read the line you entered!");

    // owner
    print!("Owner of the directory [www-data]: ");
    io::stdout().flush().unwrap();
    let mut owner = String::new();
    io::stdin().read_line(&mut owner)
        .expect("ERROR: Failed to read the line you entered!");
    // trim() because we get the carriage return in the string...
    owner = owner.trim().to_string();
    if owner.len() == 0 {
        owner = "www-data".to_string(); // default
    }

    // password
    print!("The server password: ");
    io::stdout().flush().unwrap();
    let password = read_password().unwrap();
    println!("The password is: '{}'", password);

}
