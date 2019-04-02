extern crate flate2;
extern crate regex;

use std::io;
use std::io::prelude::*;
use flate2::read::GzDecoder;
use regex::Regex;
use std::collections::HashSet;


fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock(); // or just open any normal file


    let re = Regex::new(r####"^(?P<client>[\d\.]+) ((?P<proxyclient>[\d\.]+)|-) (((?P<cookie>[a-z,A-Z,0-9,_,-]*)[:,.](?P<appserver>[a-z,A-Z,0-9]*))|-) \[(?P<time>.*?)\] "(?P<verb>\S*) (?P<vhost>[a-z,A-Z,0-9,.,_,-]*)/(?P<aplicacion>[a-z,A-Z,0-9,.,_,-]*)/*(?P<request>.*?)\s* (?P<httpversion>[A-Z]\S*)" (?P<response>\d+) (?P<bytes>\S*) (?P<duration>\S*)\s*$"####).unwrap();

//    let d = GzDecoder::new(stdin); //.expect("couldn't decode gzip stream");

//    for line in io::BufReader::new(d).lines() {
    for line in io::BufReader::new(stdin).lines() {
        let line = line.expect("Unable to read line");
        println!("{}", line);
        let caps = re.captures(&line).unwrap();
        println!("{:#?}", &caps);
        println!("{}", &caps["client"]);
    }
}
