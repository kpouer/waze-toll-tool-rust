extern crate core;

mod price_grid;
mod io_tools;
mod toll_file;
mod name_normalizer;
mod price_service;
mod price;
mod category;

pub(crate) const DEFAULT_YEAR: u16 = 2019;

fn usage() {
    println!("waze-toll-tool build-matrix <toll-file.json>");
    println!("waze-toll-tool get-prices <entry_name>");
    println!("waze-toll-tool check-prices");
}

fn command_build_matrix(args: &Vec<String>) {
    if args.len() < 3 {
        usage();
        std::process::exit(exitcode::USAGE);
    }
    let toll_file = &args[2];
    let price_service = price_service::PriceService::new();
    price_service.build_matrix(toll_file);
    std::process::exit(exitcode::OK);
}

fn command_get_prices(args: &Vec<String>) {
    if args.len() < 3 {
        usage();
        std::process::exit(exitcode::USAGE);
    }
    let entry_name = &args[2];
    let price_service = price_service::PriceService::new();
    price_service.get_prices(entry_name);
    std::process::exit(exitcode::OK);
}

fn command_check_prices() {
    let price_service = price_service::PriceService::new();
    println!("Price service loaded : {}", price_service);
    std::process::exit(exitcode::OK);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        usage();
        std::process::exit(exitcode::USAGE);
    }

    let first_arg = &args[1];
    if first_arg == "build-matrix" {
        command_build_matrix(&args);
    } else if first_arg == "get-prices" {
        command_get_prices(&args);
    } else if first_arg == "check-prices" {
        command_check_prices();
    }
    usage();
}
