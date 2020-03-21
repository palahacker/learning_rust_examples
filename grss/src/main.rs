
// This code explore differents way to make web requests with reqwest library

#[macro_use]
extern crate error_chain;
use structopt::StructOpt;
use reqwest;
use std::io::Read;
use futures::executor::block_on;
use futures::join;
use http::Method;
use tokio;



error_chain ! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

/// Struct for parsing CLI arguments with StructOpt
#[derive(StructOpt)]
struct Cli {
    /// Activate debug mode
    #[structopt(short="d", long="debug")]
    _debug: bool,
    
    /// The pattern to look for
    #[structopt(short="u", long="url")]
    _url: String,
 
}

#[tokio::main]
async fn main() {

    // Get CLI arguments
    let _args = Cli::from_args();

    // Blocking web request
    //let _future_1= http_request_1(_args._url.to_string());
    //block_on(_future_1);

    // Async simple get
    let _future_2= http_request_2(_args._url.to_string());
    block_on(_future_2);
    
    // Async web client 
    let _future_3 = http_request_3(_args._url.to_string());
    block_on(_future_3);

}

// HTTP GET request function - Blocking
fn http_request_1(_site: String) -> Result<() > {
    

    let mut _result = reqwest::blocking::get(&_site)?;
    let mut _status = String::new();
    
    _status = _result.status().to_string();
    let _body = _result.text();

    println!("Status = {:?} \n",_status);
    
    

    Ok(())
}

// HTTP GET request function - Async
async fn http_request_2(_site: String) -> Result<()>{


    let mut _result = reqwest::get(&_site).await?;
    let mut _status = String::new();
    _status = _result.status().to_string();
    println!("Status = {:?} \n",_status);

    Ok(())
}

// HTP GET request using reqwest client
async fn http_request_3(_site: String) -> Result<()>{

    
    let mut _client = reqwest::Client::new();
    let mut _request_builder = reqwest::Client::request(&_client,http::Method::GET,&_site);
    let mut _request = _request_builder.build()?;
    let mut _response = _client.execute(_request).await?;

    println!("Status = {:?} \n Headers = {:?} ",_response.status(),_response.headers());

    Ok(())

 }

    



