#[macro_use]
extern crate hyper;
extern crate url;
extern crate chrono;
extern crate time;
extern crate crypto;
extern crate rustc_serialize as serialize;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate clap;

use clap::{Arg, App};

mod azure_error;
use azure_error::AzureError;
use azure_error::UnexpectedHTTPResult;

use hyper::header::{Headers, ContentLength};
use hyper::status::StatusCode;

use time::Duration;

use std::ops::Add;
use serialize::base64::{STANDARD, ToBase64};

use url::percent_encoding::{utf8_percent_encode, HTTP_VALUE_ENCODE_SET, FORM_URLENCODED_ENCODE_SET};

use crypto::sha2::Sha256;
use crypto::hmac::Hmac;
use crypto::mac::Mac;

use std::fs;

use std::io::Read;

header! { (Authorization, "Authorization") => [String] }

pub fn submit_event(namespace: &str,
                    event_hub: &str,
                    policy_name: &str,
                    key: &str,
                    event_body: (&mut Read, u64),
                    duration: Duration)
                    -> Result<(), AzureError> {

    // prepare the url to call
    let url = format!("https://{}.servicebus.windows.net/{}/messages",
                      namespace,
                      event_hub);
    let url = try!(url::Url::parse(&url));
    debug!("url == {:?}", url);

    // create content

    // generate sas signature based on key name, key value, url and duration.
    let sas = generate_signature(&policy_name, &key, &url.to_string(), duration);
    debug!("sas == {}", sas);

    // add required headers (in this case just the Authorization and Content-Length).
    let client = hyper::client::Client::new();
    let mut headers = Headers::new();
    headers.set(Authorization(sas));
    headers.set(ContentLength(event_body.1));


    let body = hyper::client::Body::SizedBody(event_body.0, event_body.1);

    // Post the request along with the headers and the body.
    let mut response = try!(client.post(url).body(body).headers(headers).send());
    info!("response.status == {}", response.status);
    debug!("response.headers == {:?}", response.headers);

    if response.status != StatusCode::Created {
        debug!("response status unexpected, returning Err");
        let mut resp_s = String::new();
        try!(response.read_to_string(&mut resp_s));
        return Err(AzureError::UnexpectedHTTPResult(UnexpectedHTTPResult::new(StatusCode::Created, response.status, &resp_s)));
    }

    debug!("response status ok, returning Ok");
    Ok(())
}

pub fn generate_signature(policy_name: &str, hmac_key: &str, url: &str, ttl: Duration) -> String {
    let expiry = chrono::UTC::now().add(ttl).timestamp();
    debug!("expiry == {:?}", expiry);

    let url_encoded = utf8_percent_encode(url, HTTP_VALUE_ENCODE_SET);
    debug!("url_encoded == {:?}", url_encoded);

    let str_to_sign = format!("{}\n{}", url_encoded, expiry);
    debug!("str_to_sign == {:?}", str_to_sign);

    let mut v_hmac_key: Vec<u8> = Vec::new();
    v_hmac_key.extend(hmac_key.as_bytes());
    let mut hmac = Hmac::new(Sha256::new(), &v_hmac_key);
    hmac.input(str_to_sign.as_bytes());
    let sig = hmac.result().code().to_base64(STANDARD);
    let sig = utf8_percent_encode(&sig, FORM_URLENCODED_ENCODE_SET);
    debug!("sig == {:?}", sig);

    format!("SharedAccessSignature sr={}&sig={}&se={}&skn={}",
            &url_encoded,
            sig,
            expiry,
            policy_name)
}

fn main() {
    env_logger::init().unwrap();
    let matches = App::new("pusheventhub")
                      .version("0.1.0")
                      .author("Francesco Cogno <francesco.cogno@outlook.com>")
                      .about("sends standard input to event hub")
                      .arg(Arg::with_name("NAMESPACE")
                               .short("n")
                               .long("ns")
                               .help("Azure Service bus namespace")
                               .required(true)
                               .takes_value(true))
                      .arg(Arg::with_name("EVENT_HUB")
                               .short("e")
                               .long("eh")
                               .help("Azure Event Hub name")
                               .required(true)
                               .takes_value(true))
                      .arg(Arg::with_name("POLICY_NAME")
                               .short("p")
                               .long("pn")
                               .help("Azure shared access signature policy name")
                               .required(true)
                               .takes_value(true))
                      .arg(Arg::with_name("POLICY_KEY")
                               .short("k")
                               .long("pk")
                               .help("Azure shared access signature key")
                               .required(true)
                               .takes_value(true))
                      .arg(Arg::with_name("INPUT_FILE_NAME")
                               .short("f")
                               .long("fn")
                               .help("Event content file name")
                               .required(true)
                               .takes_value(true))
                      .get_matches();

    let namespace = matches.value_of("NAMESPACE").unwrap();
    let event_hub = matches.value_of("EVENT_HUB").unwrap();
    let policy_name = matches.value_of("POLICY_NAME").unwrap();
    let key = matches.value_of("POLICY_KEY").unwrap();
    let file_name = matches.value_of("INPUT_FILE_NAME").unwrap();

    let metadata = fs::metadata(file_name).unwrap();
    let mut file_handle = fs::File::open(file_name).unwrap();

    submit_event(namespace,
                 event_hub,
                 policy_name,
                 key,
                 (&mut file_handle, metadata.len()),
                 Duration::hours(1))
        .unwrap();
}
