#![crate_name = "uu_basename"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Jimmy Lu <jimmy.lu.2011@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate getopts;
extern crate libc;

#[macro_use]
extern crate uucore;

use getopts::Options;
use std::io::Write;
use std::path::{is_separator, PathBuf};

static NAME: &'static str = "basename";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn uumain(args: Vec<String>) -> i32 {
    //
    // Argument parsing
    //
    let mut opts = Options::new();
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => m,
        Err(f) => crash!(1, "Invalid options\n{}", f)
    };

    if matches.opt_present("help") {
        let msg = format!("Usage: {0} NAME [SUFFIX]\n   or: {0} OPTION\n\n\
        Print NAME with any leading directory components removed.\n\
        If specified, also remove a trailing SUFFIX.", NAME);

        print!("{}", opts.usage(&msg));

        return 0;
    }

    if matches.opt_present("version") {
        println!("{} {}", NAME, VERSION);
        return 0;
    }

    // too few arguments
    if args.len() < 2 {
        crash!(
            1,
            "{0}: {1}\nTry '{0} --help' for more information.",
            NAME,
            "missing operand"
        );
    }
    // too many arguments
    else if args.len() > 3 {
        crash!(
            1,
            "{0}: extra operand '{1}'\nTry '{0} --help' for more information.",
            NAME,
            args[3]
        );
    }

    //
    // Main Program Processing
    //

    let mut name = strip_dir(&args[1]);

    if args.len() > 2 {
        let suffix = args[2].clone();
        name = strip_suffix(name.as_ref(), suffix.as_ref());
    }

    println!("{}", name);

    0
}

fn strip_dir(fullname: &str) -> String {
    // Remove all platform-specific path separators from the end
    let mut path: String = fullname.chars().rev().skip_while(|&ch| is_separator(ch)).collect();

    // Undo reverse
    path = path.chars().rev().collect();

    // Convert to path buffer and get last path component
    let pb = PathBuf::from(path);
    match pb.components().last() {
        Some(c) => c.as_os_str().to_str().unwrap().to_owned(),
        None => "".to_owned()
    }
}

fn strip_suffix(name: &str, suffix: &str) -> String {
    if name == suffix {
        return name.to_owned();
    }

    if name.ends_with(suffix) {
        return name[..name.len() - suffix.len()].to_owned();
    }

    name.to_owned()
}
