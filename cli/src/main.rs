// Cli for Marco-Polo

use clap::Parser;
#[derive(Parser)]
#[clap(version="1.0",author="Dewa sahu",about="Marco-Polo-Cli")]

struct Cli{
    #[clap(subcommand)]
    command:Option<Commands>
}

#[derive(Parser)]
enum Commands{
    #[clap(version="1.0",author="Dewa sahu",about="Marco-Polo-Cli")]
    Play{
        #[clap(short,long)]
        name:String
    }

}


fn main(){
    let args=Cli::parse();
    match args.command{
        Some(Commands::Play { name })=>{
            let rs = cli::marco_polo(&name);
            println!("{}",rs)
        }
        None => println!("found nothing kindly pass --name parameter")
    }
}

