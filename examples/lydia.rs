use coffeehouse::lydia::LydiaAI;
use std::{
    io,
    process
};

//include tokio = { version = "1.2", features = ["full"] }
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key = "Your Access Key";

    //create a new lydia session
    let mut lydia = LydiaAI::new(access_key);

    //check if the lydia session was created, quit if errored
    match lydia.create_session().await {
        Ok(_) => println!("Session started!"),
        Err(err) => {
            eprintln!("{:#?}", err);
            process::exit(1);
        },
    }

    loop {
        //take user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //print response or exit if error
        match lydia.think_thought(input).await {
            Ok(thought) => println!("Lydia: {}", thought.results.output),
            Err(err) => {
                eprintln!("{:#?}", err);
                process::exit(1);
            },
        }
    }
}

