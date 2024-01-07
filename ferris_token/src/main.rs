use serde::{Deserialize, Serialize};
use serde_json::{json, Number};
// use solana_account_decoder::parse_account_data::ParsedAccount;
use std::str::FromStr;

use solana_client::{
    rpc_client::RpcClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    rpc_filter::{Memcmp, RpcFilterType},
    rpc_request::RpcRequest,
    rpc_response::RpcKeyedAccount,
};
use solana_sdk::{
    // account_info::AccountInfo,
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
};

#[derive(Serialize, Deserialize)]
struct TokenAmount {
    amount: String,
    decimals: Number,
    #[serde(rename = "uiAmount")]
    ui_amount: Number,
    #[serde(rename = "uiAmountString")]
    ui_amount_string: String,
}

#[derive(Serialize, Deserialize)]
struct AccountInfoB {
    #[serde(rename = "isNative")]
    is_native: bool,
    mint: String,
    owner: String,
    state: String,
    #[serde(rename = "tokenAmount")]
    token_amount: TokenAmount,
}

#[derive(Serialize, Deserialize)]
struct AccountData {
    info: AccountInfoB,
    #[serde(rename = "type")]
    type_: String,
    // space: Number,
}

fn main() {
    println!("Fetching accounts with non zero token accounts...");

    // Use your RPC url here
    let rpc = RpcClient::new("https://your_rpc_url_here".to_string());

    let token_account = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();
    // FERRIS CA
    let mint_account = Pubkey::from_str("4M1cAJY21mzQSnoGu9gNW8dE7gf9ArJCi5VWoCLH1w1v").unwrap();

    let filters = vec![
        RpcFilterType::DataSize(165),
        RpcFilterType::Memcmp(Memcmp::new_base58_encoded(0, &mint_account.to_bytes())),
    ];
    let filters = RpcProgramAccountsConfig {
        filters: Some(filters),
        account_config: RpcAccountInfoConfig {
            encoding: Some(solana_account_decoder::UiAccountEncoding::JsonParsed), // This is necessary to avoid
            data_slice: None,
            commitment: Some(CommitmentConfig {
                commitment: CommitmentLevel::Finalized,
            }),
            min_context_slot: None,
        },
        ..Default::default()
    };

    let all_user_accounts = rpc
        .send::<Vec<RpcKeyedAccount>>(
            RpcRequest::GetProgramAccounts,
            json!([token_account.to_string(), filters]),
        )
        .unwrap();

    all_user_accounts.iter().for_each(|acc| {
        // let account_data: ParsedAccount = serde_json::from_value(acc.account.data).unwrap();

        match &acc.account.data {
            solana_account_decoder::UiAccountData::LegacyBinary(_) => todo!(),
            solana_account_decoder::UiAccountData::Json(data) => {
                let parseddata =
                    serde_json::from_value::<AccountData>(data.parsed.clone()).unwrap();

                let ui_amount = parseddata.info.token_amount.ui_amount.as_f64();

                match ui_amount {
                    Some(amnt) => {
                        if amnt > 0.0 {
                            println!(
                                "{} {:?}",
                                acc.pubkey.to_string(),
                                parseddata.info.token_amount.ui_amount_string.to_string()
                            );
                        }
                    }
                    None => {
                        return {};
                    }
                }
            }
            solana_account_decoder::UiAccountData::Binary(_, _) => todo!(),
        }
    });
}
