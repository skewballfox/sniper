mod config;
mod target;
mod sniper;
mod snippet;
mod snippetParser;
mod rifle;
//mod server;
//mod server.rs



fn main() {
    //let config=config::ConfigLoader();
    let mut sniper_session=sniper::Sniper::new();
    sniper_session.add_target("12345","test.py","python");
    println!("{:#?}",sniper_session.rifle.snippets);
    println!("{:?}",sniper_session.config.languages["python"]);
    println!("{:#?}",sniper_session.snipe("python","if/elif/else"));
    println!("{:#?}",sniper_session.snipe("python","if"));
    sniper_session.add_target("12345","test.py","python");
    
    //println!("{:?}",config)
    //let config=SniperConfig::new("")
    //println!("Hello, world!");
    
   
}
