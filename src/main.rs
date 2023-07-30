use clap::Parser;
use anyhow::{bail,Result};
use encryptor::password::generate_password;
#[derive(Parser,Debug)]
#[clap(version,about,long_about = None)]
struct Args{
    #[clap(short,long)]
    seed:String,
  
    #[clap(short,long,default_value_t = 16)]
    length:usize
}
fn main() -> Result<()>{
    let args = Args::parse();
    println!("Hello {}!", args.seed);

    if args.seed.len() < 4{
        bail!("seed {} length must >= 4", &args.seed);
    } 

    let(seed,length) = (args.seed,args.length);
    let passwd = generate_password(&seed[..], length);
    match passwd {
        Ok(val) => println!("{}",val),
        Err(err) => println!("{}",err),
    }
    Ok(())
}
