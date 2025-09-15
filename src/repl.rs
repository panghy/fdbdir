use crate::util::{display_path, parse_path};
use anyhow::Result;
use foundationdb::directory::{Directory, DirectoryLayer};
use owo_colors::OwoColorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Editor;
use rustyline::{Context, Helper};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tokio::task;

struct ReplHelper {
    db: Arc<foundationdb::Database>,
    cwd: Arc<Mutex<Vec<String>>>,
}

impl Helper for ReplHelper {}
impl Validator for ReplHelper {}
impl Highlighter for ReplHelper {}
impl Hinter for ReplHelper {
    type Hint = String;
}

impl Completer for ReplHelper {
    type Candidate = Pair;
    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let commands = ["help", "exit", "quit", "pwd", "cd", "ls", "scan"];
        let parts = shell_words::split(line).unwrap_or_else(|_| vec![line.to_string()]);
        let is_space_term = line.ends_with(' ');

        // If empty or still typing command, complete commands
        if parts.len() == 0 || (!is_space_term && parts.len() == 1) {
            let start = 0;
            let prefix = line.trim_start();
            let mut out = vec![];
            for &cmd in &commands {
                if cmd.starts_with(prefix) {
                    out.push(Pair {
                        display: cmd.to_string(),
                        replacement: cmd.to_string(),
                    });
                }
            }
            return Ok((start, out));
        }

        // Path completion for cd/ls/scan first argument
        let cmd = &parts[0];
        if ["cd", "ls", "scan"].contains(&cmd.as_str()) {
            // Determine current (possibly partial) token
            let token = if is_space_term {
                ""
            } else {
                parts.last().map(|s| s.as_str()).unwrap_or("")
            };
            let base_path = if token.starts_with('/') {
                parse_path(token)
            } else {
                let mut p = self.cwd.lock().unwrap().clone();
                p.extend(parse_path(token));
                p
            };
            // Directory listing for parent and filter on last segment
            let (parent, needle) = if token.ends_with('/') {
                (base_path.clone(), "".to_string())
            } else {
                let mut parent = base_path.clone();
                let needle = parent.pop().map(|s| s).unwrap_or_default();
                (parent, needle)
            };

            let db = self.db.clone();
            let parent_for_run = parent.clone();
            let fut = async move {
                db.run(|trx, _| {
                    let parent = parent_for_run.clone();
                    async move {
                        let dl = DirectoryLayer::default();
                        let items = dl.list(&trx, &parent).await?;
                        Ok::<_, foundationdb::FdbBindingError>(items)
                    }
                })
                .await
            };
            let items =
                match task::block_in_place(|| tokio::runtime::Handle::current().block_on(fut)) {
                    Ok(v) => v,
                    Err(_) => vec![],
                };
            let mut pairs = vec![];
            let add_slash = token.ends_with('/');
            for name in items {
                if needle.is_empty() || name.starts_with(&needle) {
                    let rep = if token.starts_with('/') {
                        let base = if parent.is_empty() {
                            String::from("/")
                        } else {
                            format!("/{}/", parent.join("/"))
                        };
                        if add_slash {
                            format!("{}{}/", base, name)
                        } else {
                            format!("{}{}", base, name)
                        }
                    } else {
                        if add_slash {
                            format!("{}/", name)
                        } else {
                            name.clone()
                        }
                    };
                    pairs.push(Pair {
                        display: format!("{}/", name),
                        replacement: rep,
                    });
                }
            }
            // start position for replacement: at beginning of last token
            let start = line
                .rfind(|c| c == ' ' || c == '\t')
                .map(|i| i + 1)
                .unwrap_or(0);
            return Ok((start, pairs));
        }

        Ok((0, vec![]))
    }
}

pub async fn run_repl(db: foundationdb::Database) -> Result<()> {
    let db = Arc::new(db);
    let mut rl: Editor<ReplHelper, _> = Editor::new()?;
    let cwd_shared: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let helper: ReplHelper = ReplHelper {
        db: db.clone(),
        cwd: cwd_shared.clone(),
    };
    rl.set_helper(Some(helper));

    // History file path: ~/.fdbdir_history
    let hist_path: PathBuf = dirs::home_dir()
        .map(|p| p.join(".fdbdir_history"))
        .unwrap_or_else(|| PathBuf::from(".fdbdir_history"));
    let _ = rl.load_history(&hist_path);
    let mut cwd: Vec<String> = vec![];

    println!("fdbdir interactive. Type 'help' for commands.\n");

    loop {
        let prompt = format!("fdb:{}> ", display_path(&cwd).bold());
        let line = match rl.readline(&prompt) {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                continue;
            }
            Err(ReadlineError::Eof) => break,
            Err(e) => return Err(e.into()),
        };
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        rl.add_history_entry(line)?;

        let mut parts = shell_words::split(line).unwrap_or_else(|_| vec![line.to_string()]);
        let cmd = parts.remove(0);
        match cmd.as_str() {
            "help" => print_help(),
            "quit" | "exit" => break,
            "pwd" => println!("{}", display_path(&cwd)),
            "cd" => {
                let target = parts.get(0).map(|s| s.as_str()).unwrap_or("/");
                let new_path = if target == "/" {
                    vec![]
                } else if target == ".." {
                    let mut p = cwd.clone();
                    p.pop();
                    p
                } else if target.starts_with('/') {
                    parse_path(target)
                } else {
                    let mut p = cwd.clone();
                    p.extend(parse_path(target));
                    p
                };

                // Validate by attempting to open
                let ok = match db
                    .run(|trx, _| {
                        let path = new_path.clone();
                        async move {
                            let dl = DirectoryLayer::default();
                            let exists = dl.exists(&trx, &path).await?;
                            Ok(exists)
                        }
                    })
                    .await
                {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{} {}", "error:".red().bold(), format!("{:?}", e));
                        false
                    }
                };
                if ok {
                    cwd = new_path;
                    *cwd_shared.lock().unwrap() = cwd.clone();
                } else {
                    println!("No such directory: {}", display_path(&new_path));
                }
            }
            "ls" => {
                let target = parts.get(0).map(|s| s.as_str());
                let path = match target {
                    None => cwd.clone(),
                    Some(p) if p == "." => cwd.clone(),
                    Some(p) if p == ".." => {
                        let mut t = cwd.clone();
                        t.pop();
                        t
                    }
                    Some(p) if p.starts_with('/') => parse_path(p),
                    Some(p) => {
                        let mut t = cwd.clone();
                        t.extend(parse_path(p));
                        t
                    }
                };

                if let Err(e) = crate::util::ls_path(&db, path).await {
                    eprintln!("{} {}", "error:".red().bold(), format!("{:?}", e));
                }
            }
            "scan" | "dump" => {
                // Parse optional [limit] and/or [prefix]
                let mut limit: usize = 50;
                let mut prefix: Option<Vec<u8>> = None;
                let mut raw = false;
                for tok in parts.iter() {
                    if tok == "--raw" || tok == "-r" || tok == "raw" {
                        raw = true;
                        continue;
                    }
                    if let Ok(n) = tok.parse::<usize>() {
                        limit = n;
                        continue;
                    }
                    if prefix.is_none() {
                        if let Ok(b) = crate::util::parse_bytes_literal(tok) {
                            prefix = Some(b);
                        }
                    }
                }

                if let Err(e) = crate::util::scan_path(&db, cwd.clone(), limit, prefix, raw).await {
                    eprintln!("{} {}", "error:".red().bold(), format!("{:?}", e));
                }
            }
            other => {
                println!("Unknown command: {other}. Try 'help'.");
            }
        }
    }
    // Save history on exit
    let _ = rl.save_history(&hist_path);
    Ok(())
}

fn print_help() {
    println!("Commands:");
    println!("  help                 Show this help");
    println!("  exit | quit          Exit the REPL");
    println!("  pwd                  Print current directory path");
    println!("  cd <path>            Change directory (use /, .., or relative)");
    println!("  ls [path]            List subdirectories at path (default: current)");
    println!("  scan [limit]         Print key=>value pairs in current dir (default 50)");
}
