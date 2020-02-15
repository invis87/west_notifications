use std::collections::LinkedList;
use std::time::Duration;

use lettre::{SmtpClient, Transport};
use lettre::SendableEmail;
use lettre::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use lettre_email::EmailBuilder;
use serde::{Deserialize, Serialize};
use tokio::time::delay_for;
use chrono::{DateTime, Local};

#[derive(Serialize, Deserialize, Debug)]
struct OrderBookResponse {
    bids: LinkedList<AmountAndPrice>,
    asks: LinkedList<AmountAndPrice>
}
#[derive(Serialize, Deserialize, Debug)]
struct AmountAndPrice {
    amount: i64,
    price: i64
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    start().await;
    Ok(())
}

async fn start() -> ! {

    loop {
        let current_price = get_current_WEST_price().await;
        match current_price {
            Ok(price) => send_price_email(price),
            Err(_) => println!("error happens"),
        };
        delay_for(Duration::from_secs(5)).await;
    }

}

struct Price {
    bid: f64,
    ask: f64,
}

// WEST:  "4LHHvYGNKJUg5hj65aGD5vgScvCBmLpdRFtjokvCjSL8"
// WAVES: "WAVES"
static WEST_QUERY: &str = "https://matcher.waves.exchange/matcher/orderbook/4LHHvYGNKJUg5hj65aGD5vgScvCBmLpdRFtjokvCjSL8/WAVES?depth=5";
static EMAIL: &str = "<your email>";
static PASSWORD: &str = "<your password>";
static TOPIC: &str = "WEST price";

#[allow(non_snake_case)]
async fn get_current_WEST_price() -> Result<Price, Box<dyn std::error::Error>> {
    let resp: OrderBookResponse = reqwest::get(WEST_QUERY)
        .await?
        .json::<OrderBookResponse>()
        .await?;

    let asks_price: i64 = resp.asks.iter().map(|ask| ask.price).sum();
    let bids_price: i64 = resp.bids.iter().map(|bid| bid.price).sum();

    let ask = to_double(asks_price);
    let bid = to_double(bids_price);

    Ok(Price {bid, ask})
}

fn send_price_email(price: Price) {
    let cred_email: String = EMAIL.to_string();
    let cred_password: String = PASSWORD.to_string();

    let creds: Credentials = Credentials::new(
        cred_email,
        cred_password,
    );

    let mut mailer: SmtpTransport = SmtpClient::new_simple("smtp.gmail.com").unwrap().credentials(creds).transport();

    let text = format!("bids: {}, asks: {}", price.bid, price.ask);
    let email: SendableEmail = EmailBuilder::new()
        .to(EMAIL)
        .from(EMAIL)
        .subject(TOPIC)
        .text(text)
        .build()
        .unwrap()
        .into();

    let result = mailer.send(email);
    let now: DateTime<Local> = Local::now();

    if result.is_ok() {
        println!("{} Email sent", now);
    } else {
        println!("{} Could not send email: {:?}", now, result);
    }
}

fn to_double(amount: i64) -> f64 {
    amount as f64 / 10.0_f64.powf(8.)
}