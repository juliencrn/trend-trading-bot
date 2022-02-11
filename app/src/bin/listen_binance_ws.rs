use app::models::binance::Candlestick;
use tungstenite::connect;
use url::Url;

// See: https://binance-docs.github.io/apidocs/spot/en/#kline-candlestick-streams
static BINANCE_WS_API: &str = "wss://stream.binance.com:9443";

// Connect to binance and listen using web-socket.
// Receive prices & volumes parsed as Kline like.
fn main() {
    let endpoint = "btcusdt@kline_1m";
    let binance_url = Url::parse(&format!("{}/ws/{}", BINANCE_WS_API, endpoint)).unwrap();
    let (mut socket, response) = connect(binance_url).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");

    for (ref header, header_value) in response.headers() {
        println!("- {}: {:?}", header, header_value);
    }

    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => s,
            _ => {
                panic!("Error getting text");
            }
        };

        let parsed: Candlestick = serde_json::from_str(&msg).expect("unable to parse json");

        println!("{:#?}", parsed);
    }
}
