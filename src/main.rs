#![allow(unused)]
use std::io::prelude::*;
use std::fs;
use std::io::BufReader;
use std::fs::{File, metadata};
use clap::Parser;
use std::{thread, time::Duration};
#[macro_use]
extern crate arraylist;

use arraylist::arl::ArrayList;

#[derive(Parser)]
struct Cli {
    #[arg(help="path of the file or folder")]
    pub path: std::path::PathBuf,
    #[arg(long="priority", short='p', default_value = "0",
	  help="The mininum level of priority you want to display")]
    pub priority: i8,
    #[arg(long="recursive", short='r', default_value = "false",
	  help="Use this if you want to get the todos of a whole folder and subfolders.")]
    pub rec_mod:   bool,
    #[arg(long="extension", short='e', default_value = "",
	  help="Use this if you want to only use this on specific extension (omit the dot)")]
    pub ext: String,
}

#[derive(Debug,Clone,PartialEq)]
struct Todo {
    priority: i8,
    line: String,
}

fn calc_priority(line:&String) -> i8 {
    let start_o = line.find("TODO").expect("could not compute priority") + 3;
    let mut prio:i8 = 0;
    let mut iterator = line.chars();
    for i in 0..start_o {
	iterator.next();
    }
    for c in iterator {
	if(c == 'O') {
	    prio += 1;
	}
	else {
	    break;
	}
    }
    prio
}

fn insert_todo(todo_array:&mut ArrayList<Todo>, todo:Todo) {
    let length = todo_array.copy().len();
    if(length == 0) {
	todo_array.add(todo);
	return;
    }
    for i in 0..length {
	if todo.priority > todo_array.copy().get(i).expect("couldn't access a todo").priority {
	    todo_array.insert(i, todo);
	    break;
	}
	if i == length-1 {
	    todo_array.add(todo);
	    break;
	}
    }
}

fn merge_todo(todo_arr: &mut ArrayList<Todo>, todo_arr2: ArrayList<Todo>) {
    let length2 = todo_arr2.copy().len();
    if length2 == 0 {
	return;
    }
    for i in 0..length2 {
	insert_todo(todo_arr, todo_arr2.copy().get(i).expect("couldn't merge todos"));
    }
}

fn print_arr(todo_array:ArrayList<Todo>) {
    for i in 0..todo_array.copy().len() {
	println!("{}",todo_array.copy().get(i).expect("couldn't print a todo").line);
    }
    
}

fn file_todoer(args:Cli) -> ArrayList<Todo> {
    if args.path.extension().is_none() || args.path.extension().unwrap().to_string_lossy().to_string() == "o"
    || (args.path.extension().unwrap().to_string_lossy().to_string() != args.ext
    && args.ext != "" ) {
	return ArrayList::<Todo>::new();
    }
    let     f      = File::open(&args.path).expect("could not read file");
    let mut reader = BufReader::new(f);
    let mut lines  = 1;
    let mut line   = String::new();
    let mut todo_arr: ArrayList<Todo> = arraylist![];
    for line in reader.lines() {
	let safe_line = line.expect("could not read line");
	if safe_line.contains("TODO") {
	    let prio = calc_priority(&safe_line);
	    if prio < args.priority {
		continue;
	    }
	     let todo = Todo{
		priority : prio,
		line     : format!("{}:{}:0: priority:{} {}", args.path.display(), lines, calc_priority(&safe_line), safe_line),
	     };
	    insert_todo(&mut todo_arr, todo);
	}
	lines += 1;
    }
    todo_arr
}

fn folder_todoer(args:Cli) -> ArrayList<Todo> {
    let mut todo_arr:ArrayList<Todo> = arraylist![];
    let md = metadata(args.path.clone()).unwrap();
    if(md.is_file()) {
	merge_todo(&mut todo_arr, file_todoer(args));
    } else if(md.is_dir()) {
	let paths = fs::read_dir(&args.path.clone()).unwrap();
	for current_path in paths {
	    let args2:Cli = Cli {
		path : current_path.expect("could not translate path back").path(),
		priority : args.priority,
		rec_mod : args.rec_mod,
		ext: args.ext.clone(),
	    };
	    merge_todo(&mut todo_arr, folder_todoer(args2));
	}
    }
    todo_arr
}

fn main() {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    }).expect("Error setting Ctrl-C handler");
    
    let args = Cli::parse();
    println!("priority: {}", &args.priority);
    println!("recursive: {}", &args.rec_mod);
    let todo_arr;
    if !args.rec_mod {
	todo_arr = file_todoer(args);
    } else {
	todo_arr = folder_todoer(args);
    }
    print_arr(todo_arr);
}
