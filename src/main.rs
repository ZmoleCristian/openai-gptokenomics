//Project: OpenAI-GPTokenomics 
//Description: A simple cli tool to calculate the price for request sent to OpenAI models
//Author website: https:tragdate.ninja
//Author: @tragDate on github tiktok and youtube
//git repo: https://github.com/tragDate/openai-gptokenomics
//License: GPL-3.0

use std::fs::File;
use std::io::{BufReader, Write, Read};
use structopt::StructOpt;
use tiktoken_rs::cl100k_base;
use tiktoken_rs::r50k_base;
use std::io::BufRead;

#[derive(Debug, StructOpt)]
#[structopt(name = "openai-gptokenomics", author ="TragDate", about)]
struct Cli {
    #[structopt(short, long, default_value = "gpt-3.5-turbo")]
    model: String,
    #[structopt(long = "request_file")]
    request_file: Option<String>,
    #[structopt(long = "request_data")]
    request_data: Option<String>,
    #[structopt(long = "answer_file")]
    answer_file: Option<String>,
    #[structopt(long = "answer_data")]
    answer_data: Option<String>,
    #[structopt(short = "a",long = "answer", conflicts_with= "request")]
    answer: bool,
    #[structopt(short = "r", long = "request", conflicts_with= "answer")]
    request : bool,
    #[structopt(short, long)]
    output_file: Option<String>,
}

fn count_tokens(text: &str, model: &str) -> usize {
    let bpe = match model {
        "gpt-4" | "gpt-3.5-turbo" => cl100k_base().unwrap(),
        "davinci" | "curie" | "babbage" |"ada" => r50k_base().unwrap(),
        _ => panic!("Unsupported model: {}", model),
    };

    bpe.encode_with_special_tokens(text).len()
}

fn calculate_price(num_tokens: usize, model: &str, operation: &str) -> f64 {
    let prices = [
        ("gpt-4", 0.03, 0.06),
        ("gpt-3.5-turbo", 0.002, 0.002),
        ("davinci", 0.02, 0.02),
        ("curie", 0.002, 0.002),
        ("babbage", 0.0005, 0.0005),
        ("ada", 0.0004, 0.0004)
    ];

    let price = match prices.iter().find(|(model_name, _, _)| model_name == &model) {
        Some((_, request_price, response_price)) => match operation {
            "request" => request_price,
            "answer" => response_price,
            _ => panic!("Invalid operation"),
        },
        None => panic!("Invalid model"),
    };

    (num_tokens as f64) * price / 1000.0
}
fn read_input(input_file: Option<&str>) -> String { //a adaugat input file ca Option sa poata sa fie true/false 
    if let Some(filename) = input_file {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let lines = reader.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
        lines.join("\n")
    } else {
        String::new()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let cli_args: Vec<String> = std::env::args().collect();

    if cli_args.len() <= 1 {
        let mut app = Cli::clap();
        app.print_help()?;
        println!();
        println!();
        println!("SUPPORTED MODELS:");
        println!(" \t \t gpt-4 \n \t \t gpt-3.5-turbo \n \t \t davinci \n \t \t ada \n \t\t babbage \n \t \t curie");
        return Ok(());
    }

    let model = &args.model;
    let req_data = if let Some(filename) = &args.request_file {
        read_input(Some(filename))
    } else if let Some(data) = &args.request_data {
        data.clone()
    } else if args.request {
             let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut text = String::new();
            handle.read_to_string(&mut text).unwrap();
            text.trim().to_owned()
    } 

    else {
        read_input(None) // a dat read input fara file
    };

    let ans_data = if args.answer || args.answer_file.is_some() || args.answer_data.is_some() {
        if let Some(filename) = &args.answer_file {
            read_input(Some(filename))
        } else if let Some(data) = &args.answer_data {
            data.clone()
        } else { // a scris asta in caz ca e --answer si sa ia din stdin ca answer
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            let mut text = String::new();
            handle.read_to_string(&mut text).unwrap();
            text.trim().to_owned()
        }
    } else {
        String::new()
    };

    let req_tokens = count_tokens(&req_data, model);
    let ans_tokens = count_tokens(&ans_data, model);

    let req_price = calculate_price(req_tokens, model, "request");
    let ans_price = calculate_price(ans_tokens, model, "answer");

    let total_price = req_price + ans_price;
    let output = format!(
        "Request price: ${:.5} for {} tokens\nAnswer price: ${:.5} for {} tokens\nTotal price: ${:.5}\n",
        req_price, req_tokens,
        ans_price, ans_tokens,
        total_price
    );

    match args.output_file {
        Some(filename) => {
            let mut file = File::create(filename)?;
            file.write_all(output.as_bytes())?;
        },
        None => {
            println!("{}", output);
        },
    }

    Ok(())
}
