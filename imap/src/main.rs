use imap;
use native_tls;
use futures::executor::block_on;
use tokio;
use structopt::StructOpt;
use std::net::ToSocketAddrs;

#[macro_use]
extern crate error_chain;

error_chain ! {
    foreign_links {
        //
    }
}

/// Struct for parsing CLI arguments with StructOpt
#[derive(StructOpt)]
struct Cli {
    /// Activate debug mode
    #[structopt(short="d", long="debug")]
    _debug: bool,
    
    /// Imap server
    #[structopt(short="i", long="imap_server")]
    _imap_srv: String,

    /// Imap server
    #[structopt(short="u", long="imap_user")]
    _imap_user: String,

    /// Imap server
    #[structopt(short="p", long="imap_pass")]
    _imap_pass: String,
 
}

#[tokio::main]
async fn main() {

    // Get CLI arguments
    let _args = Cli::from_args();

    // We start IMAP connection
    let future_1 = fetch_inbox_top(_args._imap_srv.to_string(),_args._imap_user.to_string(),
                    _args._imap_pass.to_string());
    
    let result = block_on(future_1).expect("Something went wrong with IMAP connection");

    for _body in &result {
        println!("{:?}", _body);
    }
  

}

// Function to get messages from mailbox a return a array with the message bodies
async fn fetch_inbox_top(_imap_srv: String,_imap_usr: String,_imap_pass: String) 
//            -> Result<[String;100], imap::Error> {
            -> imap::error::Result<Option<Vec<String>>> {
    // We generate connection details
    let _server_details = [_imap_srv.to_string(),":".to_string(),"993".to_string()].concat();

    // We obtain SocketAddr from server details
    let _server: Vec<_> = _server_details
    .to_socket_addrs()
    .expect("Unable to resolve domain")
    .collect();
    
    // Only valid if we are using a ipaddress in the place of a hostname
    // We obtain SocketAddr from server details
    //let _server: SocketAddr = _test
    //    .parse()
    //    .expect("Unable to parse socket address");

    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect(_server[0],_imap_srv.to_string(), &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let mut imap_session = client
        .login(_imap_usr, _imap_pass)
        .map_err(|e| e.0).unwrap();
    
        // we want to fetch the first email in the INBOX mailbox
    let _mailbox_status = imap_session.select("INBOX");

    
    let mut _n_messages = match _mailbox_status {
        Ok(_s)  => _s.exists, 
        Err(_e) => 0,
    };

    _n_messages = _n_messages + 1;

    let mut _messages;
    let mut _message;
    let mut _body;
    //let mut _header;

    let mut _message_vec: Vec<String> = Vec::new();
    
    // Going over the messages
    for n in 1.._n_messages {
        
        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        _messages = imap_session.fetch(n.to_string(), "RFC822")?;

        // Delete message
        //imap_session.store(format!("{}", "1"), "+FLAGS (\\Deleted)")?;
        //imap_session.expunge()?;

        _message = if let Some(m) = _messages.iter().next() {
            m
        } else {
            return Ok(None);
        };

        // extract the message's header
        //_header = _message.header().expect("message did not have a header");
    
        // extract the message's body
        _body = _message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(_body)
            .expect("message was not valid utf-8")
            .to_string();

        // Add message body to the messages vector
        _message_vec.push(body);
    }

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(Some(_message_vec))
}
