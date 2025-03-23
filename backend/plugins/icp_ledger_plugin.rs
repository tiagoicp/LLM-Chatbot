use ic_ledger_types::{AccountIdentifier, AccountBalanceArgs, MAINNET_LEDGER_CANISTER_ID, account_balance};
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod, HttpResponse, TransformArgs,
    TransformContext,
};
use candid::{CandidType, Decode, Deserialize, Encode, Nat};

#[derive(Debug, Clone, CandidType, Deserialize)]
struct OnThisDay {
    title: String,
    year: String,
    wiki_link: String,
}


/// Lookup the balance of an ICP account.
pub async fn lookup_account(account: &str) -> String {
    if account.len() != 64 {
        return "Account must be 64 characters long".to_string();
    }

    match AccountIdentifier::from_hex(account) {
        Ok(account) => {
            let balance = account_balance(
                MAINNET_LEDGER_CANISTER_ID,
                AccountBalanceArgs {
                    account,
                }
            ).await.expect("call to ledger failed");

            format!("Balance of {} is {} ICP", account, balance)
        }
        Err(_) => "Invalid account".to_string(),
    }
}

/// Get the current price of ICP from Coingecko.
pub async fn get_price() -> Result<f64, String>  {
    
    // Generate URL. Target must support IPv6.
    let url = format!("https://byabbe.se/on-this-day/april/01/events.json");

    // TransformContext is used to specify how the HTTP response is processed before consensus tries to agree on a response.
    // This is useful to e.g. filter out timestamps/sessionIDs out of headers that will be different across the responses the different replicas receive.
    // If the data (including status, headers and body) they receive does not match across the nodes, the canister will reject the response!
    // You can read more about it here: https://internetcomputer.org/docs/current/developer-docs/smart-contracts/advanced-features/https-outcalls/https-outcalls-how-to-use.
    let transform_context = TransformContext::from_name("transform".to_string(), vec![]);
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None, // Can be set to limit cost. Our response has no predictable size, so we set no limit.
        headers: vec![],
        transform: Some(transform_context),
    };

    fn http_response_to_on_this_day(http: &str) -> Option<OnThisDay> {
        let json: serde_json::Value = serde_json::from_str(&http).ok()?;
        let title = json["events"][0]["description"].as_str()?;
        let year = json["events"][0]["year"].as_str()?;
        let wiki_link = json["events"][0]["wikipedia"][0]["wikipedia"].as_str()?;
        Some(OnThisDay {
            title: title.to_string(),
            year: year.to_string(),
            wiki_link: wiki_link.to_string(),
        })
    }

    // Perform HTTPS outcall using roughly 100B cycles. 
    // See https outcall cost calculator: https://7joko-hiaaa-aaaal-ajz7a-cai.icp0.io.
    // Unused cycles are returned.
    let quote = match http_request(request, 100_000_000_000).await {
        Ok((response,)) => {
            let body_string =
                String::from_utf8(response.body).expect("Response is not UTF-8 encoded.");
            let Some(otd) = http_response_to_on_this_day(&body_string) else {
                return Err(format!("Failed get event for date"));
            };
            otd
        }
        Err(err) => {
            return Err(format!("http_request resulted in an error: {err:?}"));
        }
    };

    Ok(5.1)
}