
/// Formats a url of connection to the gateway
pub fn get_gateway_url (compress: bool, encoding: &str, v: i16) -> String {
    return format!(
        "wss://gateway.discord.gg/?v={}&encoding={}&compress={}",
        v, encoding,
        if compress { "zlib-stream" } else { "" }
    );
}