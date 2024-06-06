use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header,Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinner::{SpinnerBuilder,SpinnerHandle};
use std::env;
use std::io::{stdin,stdout, Write};

fn main() {
    println!("Hello, world!");

    //a struct to work with the API response
    #[derive(Deserialize, Debug)]
    struct OAIResponse {
        id: Option<String>,
        object: Option<String>,
        created: Option<u64>,
        model: Option<String>,
        choices: Vec<OAIChoices>,
    }

    //a struct for the choices
    #[derive(Deserialize, Debug)]
    // struct Choices {
    //     prompt: String,
    //     max_tokens: u8,
    //     temperature: f32,
    //     top_p: f32,
    //     frequency_penalty: f32,
    //     presence_penalty: f32,
    // }
    struct OAIChoices {
        text: String,
        index: u8,
        logprobs: Option<u8>,
        finish_reason: String,
    }

    //a struct for the request you will make to the API
    #[derive(Serialize, Debug)]
    struct OAIRequest {
        prompt: String,
        max_tokens: u16,
        // temperature: f32,
        // top_p: f32,
        // frequency_penalty: f32,
        // presence_penalty: f32,
    }

//tokio async main function
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error + send + Sync>> {
    //Load my env variables
    dotenv().ok();
    // create a Httpconnector, hyper
    let https = HttpsConnector::new();
    // create a client
    let client = Client::builder().build::<_, hyper::Body>(https);
    //URL to which we will make the request
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    // preamble, prompt to chatGPT
    let preamble = "The following is a conversation with an AI assistant. The assistant is helpful, creative, clever, and very friendly.";
    //token, in the header
    let oai_token: String = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);

    //Loop, inside the Loop a way to read user input
    loop {
        print!("You: ");
        stdout().flush().unwrap();
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();
        //spinner, wait for the response
        let spinner = SpinnerBuilder::new("Thinking...".to_string()).start();
        // request to chatGPT for every single user input, Loop
        let oai_request = OAIRequest {
            prompt: format!("{}{}", preamble, user_input),
            max_tokens: 150,
            // temperature: 0.5,
            // top_p: 1.0,
            // frequency_penalty: 0.0,
            // presence_penalty: 0.0,
        };
        let req = Request::post(uri)
            .header(header::AUTHORIZATION, auth_header_val)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&oai_request).unwrap()))
            .unwrap();
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let oai_response: OAIResponse = serde_json::from_reader(body.reader())?;
        // reponse and we print that response
        for choice in oai_response.choices {
            println!("AI: {}", choice.text);
        }
        spinner.stop();
    }

}


//token, in the header
//Loop, inside the Loop a way to read user input
//spinner, wait for the response
// request to chatGPT for every single user input, Loop
// reponse and we print that response
}

