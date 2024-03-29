use config::{Config,File};
use clap::{App,Arg,ArgMatches,SubCommand};
use std::io::Write;
mod server;
mod response_strategy;
use response_strategy::{ResponseStrategyCtor};
mod logger;
use logger::setup_logger;
mod server_config;
use server_config::ServerConfig;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("chaos-ntpd")
        .version(VERSION)
        .author("plates <plates.jsnm@gmail.com>")
        .subcommand(SubCommand::with_name("start")
                    .about("start the server")
                    .arg(Arg::with_name("config")
                         .value_name("PATH")
                         .default_value("chaos-ntpd.conf")
                         .short("c")
                         .long("config")
                         .help("use config from path")
                         .takes_value(true)))
        .subcommand(SubCommand::with_name("generate-config")
                    .about("generate the default configuration file")
                    .arg(Arg::with_name("path")
                         .value_name("PATH")
                         .default_value("chaos-ntpd.conf")
                         .required(true)
                         .help("path of the new config file")
                         .takes_value(true)))
}

fn start(args: &ArgMatches) -> std::io::Result<()> { 
    let mut config_rep: Config = Config::new();
    match args.value_of("config") {
        Some(path) => config_rep = config_rep.merge(File::with_name(path).format(config::FileFormat::Toml))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::NotFound, err))?.clone(),
        None => std::io::Result::Err(std::io::Error::new(std::io::ErrorKind::NotFound, "no config file provided".to_string()))?
    }

    //https://github.com/mehcode/config-rs/issues/57
    //not sure if merging configuration files like this is the best idea but whatever
    //let mut configured = false;
    //let mut config_files = vec![String::from("/etc/chaos-ntpd.conf"), String::from("./chaos-ntpd.conf")];
    //args.value_of("config").and_then(|c| Some(config_files.push(String::from(c))));

    //for n in config_files.iter() {
    //    match config_rep.merge(File::with_name(n)) {
    //        Ok(config) => { 
    //            println!("merged {}", n);
    //            config_rep = config.clone();
    //            configured = true;
    //        },
    //        Err(err) => {
    //            println!("couldnt load {}: {}", n, err)
    //        }
    //    }
    //}

    //if !configured {
    //    println!("no configuration files were found");
    //    return Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    //}

    let config = config_rep.try_into::<ServerConfig>().unwrap();

    let _guard = setup_logger(config.log.level);

    let rs = inventory::iter::<&dyn ResponseStrategyCtor>.into_iter().find(|s| s.name() == config.server.resp_strategy).unwrap().new_boxed();

    let mut server = server::Server {
        port: config.server.port,
        addr: config.server.address,
        log_all_requests: config.log.log_all_requests,
        response_strategy: rs,
    };
    server.start_server().map_err(|err| match err.kind() {
        std::io::ErrorKind::PermissionDenied => {
            println!("Permission to bind to port {} was denied", config.server.port);
            err
        }
        _ => err
    })
}

fn generate_config(args: &ArgMatches) -> std::io::Result<()> {
    let path = args.value_of("path").unwrap();
    let mut file = std::fs::File::create(path)?;
    let cfg = ServerConfig::default();
    let data = toml::to_string_pretty(&cfg).unwrap();
    file.write_all(data.as_bytes())?;
    println!("Generated {}", path);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let mut app = get_app();
    let args = app.clone().get_matches();

    match args.subcommand() {
        ("start", Some(sub_args)) => start(sub_args),
        ("generate-config", Some(sub_args)) => generate_config(sub_args),
        _ => {
            app.print_help().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            println!();
            Ok(())
        }
    }
}

