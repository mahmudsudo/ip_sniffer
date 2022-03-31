use std::{env::args, net::{IpAddr, TcpStream}, str::FromStr, process,sync::mpsc::{Sender,channel},thread, io::{self, Write}};

fn main() {
   let args = args().collect::<Vec<_>>();
let prog = args[0].clone();
let arguments = Args::new(&args).unwrap_or_else(|s| {
    if s.contains("help"){
        process::exit(0)
    }else{
        eprintln!("{} problem parsing arguments: {}",prog,&s);
        process::exit(0)
    }
});
let num_threads = arguments.threads;
let _addr = arguments.ipaddr;
let (tx,rx) = channel();
for i in 0..num_threads{
    let tx = tx.clone();
    thread::spawn(move ||{
        scan(tx,i,_addr,num_threads);
    });
}
let mut out = vec![];
drop(tx);
for p in &rx{
    out.push(p);
}
println!("");
out.sort();
for v in &out{
    println!("{} is open",v);
}
}
fn scan(tx:Sender<u16>,start_port:u16,addr:IpAddr,num_threads:u16) {
    let mut port= start_port+1;
    loop{
        match TcpStream::connect((addr,port)){
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_)=>{}

        }
        if MAX - port <= num_threads{
            break
        }
        port+=num_threads;
    }
}
const MAX :u16 = 65535;
#[derive(Debug, Clone)]
  #[warn(dead_code)]
struct Args{
   
    flag: String,
    ipaddr :IpAddr,
    threads:u16
}
impl Args{
    fn new(args: &[String]) -> Result<Args,&'static str>{
        if args.len() < 2 {
         return  Err("not enough arguments");
        }else if args.len()>4{
          return  Err("too much arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) =IpAddr::from_str(&f){
            return Ok(Args{flag : String::from(""),ipaddr,threads:4})
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2{
                println!("Usage: -j to select no of threads \r\n
                            -h or -help to show help message");
                            return  Err("help");
            }else if flag.contains("-h") || flag.contains("-help"){
                return  Err("too many arguments");
            }
            else if flag.contains("-j"){
                let ipaddr = match IpAddr::from_str(&args[3]){
                    Ok(s) => s,
                    Err(_) => return Err("not a valid ip address")
                };
                let threads = match args[2].parse::<u16>(){
                    Ok(s)=> s, 
                    Err(_) => return Err("failed to parse number")

                };

                return Ok(Args{threads,flag,ipaddr})

            }else {
                Err("invalid syntax")
            }
        }
    }
}