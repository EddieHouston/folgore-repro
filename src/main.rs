use esplora_api::EsploraAPI;

/// Exact copy of folgore's raw_to_num before the fix (coffee-tools/folgore#100).
/// Panics on non-numeric response instead of returning Result.
fn raw_to_num(buff: &[u8]) -> i64 {
    let buf = String::from_utf8(buff.to_vec()).expect("impossible convert the buff to a string");
    buf.parse().expect("impossible parse a string into a i64")
}

/// Mirrors folgore's sync_chain_info / sync_block_by_height call chain:
///   self.client
///       .raw_call("/blocks/tip/height")
///       .map_err(|err| error!("{err}"))
///       .map(|raw| raw_to_num(&raw))
fn get_current_height(client: &EsploraAPI) -> Result<i64, String> {
    client
        .raw_call("/blocks/tip/height")
        .map_err(|err| format!("{err}"))
        .map(|raw| {
            let buf = String::from_utf8_lossy(&raw);
            match buf.trim().parse::<i64>() {
                Ok(height) => height,
                Err(e) => {
                    eprintln!("!!! CAUGHT BAD 200 RESPONSE !!!");
                    eprintln!("  parse error: {e}");
                    eprintln!("  body length: {}", raw.len());
                    eprintln!("  body (utf8): '{buf}'");
                    eprintln!("  body (hex):  {}", raw.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" "));
                    // Now panic like old folgore would:
                    raw_to_num(&raw)
                }
            }
        })
}

fn main() {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://blockstream.info/api".to_string());

    let total: usize = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(800);

    println!("Target: {url}");
    println!("Requests: {total}");
    println!("---");

    let client = EsploraAPI::new(&url).expect("failed to create client");

    for i in 1..=total {
        match get_current_height(&client) {
            Ok(height) => {
                if i % 100 == 0 {
                    println!("[{i}/{total}] OK height={height}");
                }
            }
            Err(e) => {
                println!("[{i}/{total}] HTTP error: {e}");
            }
        }
    }

    println!("---");
    println!("Done. No panic encountered.");
}
