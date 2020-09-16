use std::process::{Command, Stdio, Child, ChildStdin};
use std::io::{Write, BufRead, BufReader};
use std::error::Error;

use std::sync::mpsc;
use std::time::Duration;
use regex::Regex;

use options::{UCIOption, UCIOptionType};


pub mod options {
    use std::fmt;

    pub enum UCIOptionType {
        Check(bool),
        Spin(isize),
        Combo(String),
        Button,
        String(String),
    }

    impl UCIOptionType {
        pub fn to_str(&self) -> String {
            match self {
                UCIOptionType::Check(val) => if *val { "true".to_string() } else { "false".to_string() },
                UCIOptionType::Button => "".to_string(),
                UCIOptionType::Spin(val) => format!("{}", val),
                UCIOptionType::Combo(val) => format!("{}", val),
                UCIOptionType::String(val) => format!("{}", val),
            }
        }
        pub fn from_str(option_type: &str, value: &str) -> Option<Self> {
            match option_type {
                "check" => Some( UCIOptionType::Check( value == "true" ) ),
                "spin" => Some( UCIOptionType::Spin( value.parse::<isize>().ok()? ) ),
                "combo" => Some( UCIOptionType::Combo( value.to_string() ) ),
                "string" => Some( UCIOptionType::String( value.to_string() ) ),
                "button" | _ => Some( UCIOptionType::Button ),
            }
        }
    }

    pub struct UCIOption {
        pub name: String,
        pub default: UCIOptionType,
        pub min: UCIOptionType,
        pub max: UCIOptionType,
        pub vars: Vec<UCIOptionType>,
    }

    impl fmt::Display for UCIOption {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "UCIOption: {}, {}", self.name, self.default.to_str())
        }
    }

    impl fmt::Debug for UCIOption {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "UCIOption: {}, {}", self.name, self.default.to_str())
        }
    }

}

#[derive(Debug)]
struct PvScoreStruct {
    score: isize,
    is_mate: bool,
}

#[derive(Debug)]
enum IdType {
    Name, Author
}

#[derive(Debug)]
enum Message {
    Invalid,
    Id { id_type: IdType, value: String },
    UciOk,
    ReadyOk,
    BestMove {best_move: String, ponder: String},
    Info{
        depth: usize,
        multipv: usize,
        pv: String,
        score: PvScoreStruct,
    },
    OptionMsg(UCIOption)
}

impl Message {
    fn from_str(line: &str) -> Option<Self> {
        lazy_static! {
            static ref PAT_ID: Regex = Regex::new(r"id name ([\w\s]+)").unwrap();
            static ref PAT_UCI_OK: Regex = Regex::new(r"uciok").unwrap();
            static ref PAT_READY_OK: Regex = Regex::new(r"readyok").unwrap();
            static ref PAT_INFO: Regex = Regex::new(
                r"info(?=.* depth (\d+))(?=.* multipv (\d+))(?=.* score (cp|mate) (\d+))(?=.* pv (\w\d\w\d)).*"
            ).unwrap();
            static ref PAT_BEST_MOVE: Regex = Regex::new(r"bestmove (\w\d\w\d).*").unwrap();
            static ref PAT_OPTION: Regex = Regex::new(
                r"option name (\w+) type (\w+) default ([\w\d]+) min ([\w\d]+) max ([\w\d]+).*"
            ).unwrap();
        }
        if let Some(caps) = PAT_ID.captures(line) {
            if caps.len() == 3 {
                Some( Message::Id{
                    id_type: if &caps[1] == "name" { IdType::Name } else { IdType::Author },
                    value: caps[2].to_string()
                })
            } else { None }
        } else if PAT_UCI_OK.is_match(line) {
            Some( Message::UciOk )
        } else if PAT_READY_OK.is_match(line) {
            Some( Message::ReadyOk )
        } else if let Some(caps) = PAT_INFO.captures(line) {
            if caps.len() == 6 {
                Some( Message::Info{
                    depth: caps[1].parse::<usize>().ok()?,
                    multipv: caps[2].parse::<usize>().ok()?,
                    pv: caps[3].to_string(),
                    score: PvScoreStruct{
                        is_mate: &caps[4] == "cp",
                        score: caps[5].parse::<isize>().ok()?,
                    },
                } )
            } else { None }
        } else if let Some(caps) = PAT_BEST_MOVE.captures(line) {
            if caps.len() == 3 {
                Some( Message::BestMove{
                    best_move: caps[1].to_string(),
                    ponder: caps[2].to_string()
                } )
            } else { None }
        } else if let Some(caps) = PAT_OPTION.captures(line) {
            if caps.len() == 6 {
                Some( Message::OptionMsg( UCIOption{
                    name: caps[1].to_string(),
                    default: UCIOptionType::from_str(&caps[2], &caps[3])?,
                    min: UCIOptionType::from_str(&caps[2], &caps[4])?,
                    max: UCIOptionType::from_str(&caps[2], &caps[5])?,
                    vars: Vec::new(),
                }) )
            } else { None }
        } else { None }
    }
}

pub struct UCIClient {
    engine: Child,
    out_reader: mpsc::Receiver<Message>
}

impl UCIClient {
    pub fn try_new(engine_exec: &str) -> Result<Self, Box<dyn Error>> {
        let mut engine = Command::new(engine_exec)
                        .stdin(Stdio::piped())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn()?;
        
        let buff = BufReader::new( 
            engine.stdout.take().ok_or("Could not capture engine process stdout")? 
        );
        let (tx, rx) = mpsc::channel::<Message>();

        std::thread::spawn(move || {
            for line in buff.lines() {
                match line {
                    Ok(line) => {
                        if let Err(e) = tx.send(
                            Message::from_str(&line).unwrap_or(Message::Invalid) 
                        ) {
                            eprintln!("send message error: {}", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("read line error: {}", e);
                    }
                }
            }
        });

        Ok( UCIClient{ engine, out_reader: rx } )
    }
    pub fn set_option(
        &mut self, option: UCIOption, value: UCIOptionType
    ) -> Result<(), Box<dyn Error>> {
        Ok (self._in()?.write_all(
            format!(
                "setoption name {} value {}\n", 
                option.name, 
                value.to_str()
            ).as_bytes()
        )?)
    }

    fn _in(&mut self) -> Result<&mut ChildStdin, &str> {
        self.engine.stdin.as_mut().ok_or("Could not capture engine process stdin")
    }

    pub fn init_uci(&mut self) -> Result<bool, Box<dyn Error>> {
        // self.clear_out()?;
        self._in()?.write_all(b"uci")?;
        while let Ok(message) = self.out_reader.recv_timeout(Duration::from_millis(500)) {
            println!("received msg: {:?}", message);
            if let Message::UciOk = message {
                return Ok( true )
            }
        }
        Err( "Timed out waiting for engine response" )?
    }
}
