mod config;
mod target;
mod sniper;
mod snippet;
//mod server.rs



fn main() {
    let config=config::ConfigLoader();
    let mut targ=target::Target::new("12345","test.py","python");
    
    
    println!("{:?}",config)
    //let config=SniperConfig::new("")
    //println!("Hello, world!");
    
   
}
