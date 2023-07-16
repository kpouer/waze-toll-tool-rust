extern crate core;

use std::process::ExitCode;
use price_service::PriceService;

mod price_grid;
mod io_tools;
mod toll_file;
mod name_normalizer;
mod price_service;
mod price;
mod category;

pub(crate) const DEFAULT_YEAR: u16 = 2019;
const USAGE: u8 = 64;

fn usage() -> ExitCode {
    println!("waze-toll-tool build-matrix <toll-file.json>");
    println!("waze-toll-tool get-prices <entry_name>");
    println!("waze-toll-tool check-prices");
    ExitCode::from(USAGE)
}

fn command_build_matrix(args: &Vec<String>) -> ExitCode {
    if args.len() < 3 {
        return usage();
    }
    let toll_file = &args[2];
    let price_service = PriceService::new();
    price_service.build_matrix(toll_file);
    ExitCode::SUCCESS
}

fn command_get_prices(args: &Vec<String>) -> ExitCode {
    if args.len() < 3 {
        return usage();
    }
    let entry_name = &args[2];
    let price_service = PriceService::new();
    price_service.get_prices(entry_name);
    ExitCode::SUCCESS
}

fn command_get_station(args: &Vec<String>) -> ExitCode {
    if args.len() < 3 {
        return usage();
    }
    let station_name = &args[2];
    let price_service = PriceService::new();
    price_service.get_station(station_name);
    ExitCode::SUCCESS
}

fn command_check_prices() -> ExitCode {
    let price_service = PriceService::new();
    println!("Price service loaded : {}", price_service);
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        return usage();
    }

    let first_arg = &args[1];
    if first_arg == "build-matrix" {
        return command_build_matrix(&args);
    } else if first_arg == "check-prices" {
        return command_check_prices();
    } else if first_arg == "get-station" {
        return command_get_station(&args);
    } else if first_arg == "get-prices" {
        return command_get_prices(&args);
    }
    usage()
}
