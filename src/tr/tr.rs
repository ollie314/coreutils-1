#![crate_name = "uu_tr"]

/*
 * This file is part of the uutils coreutils package.
 *
 * (c) Michael Gehring <mg@ebfe.org>
 * (c) kwantam <kwantam@gmail.com>
 *     20150428 created `expand` module to eliminate most allocs during setup
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

extern crate bit_set;
extern crate getopts;

#[macro_use]
extern crate uucore;

use bit_set::BitSet;
use getopts::Options;
use std::io::{stdin, stdout, BufReader, BufWriter, Read, Write};
use std::collections::HashMap;

use expand::ExpandSet;

mod expand;

static NAME: &'static str = "tr";
static VERSION: &'static str = env!("CARGO_PKG_VERSION");
const BUFFER_LEN: usize = 1024;

fn delete(set: ExpandSet, complement: bool) {
    let mut bset = BitSet::new();
    let mut stdout = stdout();
    let mut buf = String::with_capacity(BUFFER_LEN + 4);

    for c in set {
        bset.insert(c as usize);
    }

    let is_allowed = |c : char| {
        if complement {
            bset.contains(c as usize)
        } else {
            !bset.contains(c as usize)
        }
    };

    let mut reader = BufReader::new(stdin());

    while let Ok(length) = reader.read_to_string(&mut buf) {
        if length == 0 { break }

        let filtered = buf.chars()
                          .filter(|c| { is_allowed(*c) })
                          .collect::<String>();
        safe_unwrap!(stdout.write_all(filtered.as_bytes()));
        buf.clear();
    }
}

fn tr<'a>(set1: ExpandSet<'a>, mut set2: ExpandSet<'a>) {
    //let mut map = VecMap::new();
    let mut map = HashMap::new();
    let stdout = stdout();
    let mut buf = String::with_capacity(BUFFER_LEN + 4);

    let mut s2_prev = '_';
    for i in set1 {
        s2_prev = set2.next().unwrap_or(s2_prev);

        map.insert(i as usize, s2_prev);
    }

    let mut reader = BufReader::new(stdin());
    let mut writer = BufWriter::new(stdout);

    while let Ok(length) = reader.read_to_string(&mut buf) {
        if length == 0 { break }

        {
            let mut chars = buf.chars();

            while let Some(ch) = chars.next() {
                let trc = match map.get(&(ch as usize)) {
                    Some(t) => *t,
                    None => ch,
                };
                safe_unwrap!(writer.write_all(format!("{}", trc).as_ref()));
            }
        }

        buf.clear();
    }
}

fn usage(opts: &Options) {
    println!("{} {}", NAME, VERSION);
    println!("");
    println!("Usage:");
    println!("  {} [OPTIONS] SET1 [SET2]", NAME);
    println!("");
    println!("{}", opts.usage("Translate or delete characters."));
}

pub fn uumain(args: Vec<String>) -> i32 {
    let mut opts = Options::new();

    opts.optflag("c", "complement", "use the complement of SET1");
    opts.optflag("C", "", "same as -c");
    opts.optflag("d", "delete", "delete characters in SET1");
    opts.optflag("h", "help", "display this help and exit");
    opts.optflag("V", "version", "output version information and exit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(err) => {
            show_error!("{}", err);
            return 1;
        }
    };

    if matches.opt_present("help") {
        usage(&opts);
        return 0;
    }

    if matches.opt_present("version") {
        println!("{} {}", NAME, VERSION);
        return 0;
    }

    if matches.free.is_empty() {
        usage(&opts);
        return 1;
    }

    let dflag = matches.opt_present("d");
    let cflag = matches.opts_present(&["c".to_owned(), "C".to_owned()]);
    let sets = matches.free;

    if cflag && !dflag {
        show_error!("-c is only supported with -d");
        return 1;
    }

    if dflag {
        let set1 = ExpandSet::new(sets[0].as_ref());
        delete(set1, cflag);
    } else {
        let set1 = ExpandSet::new(sets[0].as_ref());
        let set2 = ExpandSet::new(sets[1].as_ref());
        tr(set1, set2);
    }

    0
}
