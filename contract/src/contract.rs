use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use secret_toolkit::crypto::sha_256;
use secret_toolkit::crypto::Prng;
// use secret_toolkit::permit::validate;
// use secret_toolkit::permit::Permit;

use crate::msg::{CardResponse, HandleMsg, InitMsg, QueryMsg};
// use crate::msg::{ ViewingKeysResponse};
// use crate::state::{SCANNED_VIEWING_KEYS, ScannedViewingKey};
use crate::state::{Card, CARD_VIEWING_KEY, ENTROPY, USER_CARDS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InitMsg,
) -> StdResult<Response> {
    ENTROPY.save(deps.storage, &msg.entropy)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: HandleMsg) -> StdResult<Response> {
    match msg {
        HandleMsg::Create { card, index } => try_create_card(deps, info, card, index),
        HandleMsg::Burn { index } => try_burn_card(deps, env, info, index),
        HandleMsg::GenerateViewingKey { index } => try_generate_viewing_key(deps, env, info, index),
        // HandleMsg::ScanViewingKey {
        //     wallet,
        //     viewing_key,
        //     index,
        // } => try_scan_viewing_key(deps, info, wallet, index, viewing_key),
    }
}

pub fn try_generate_viewing_key(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    index: u8,
) -> StdResult<Response> {
    //map for viewing keys
    let viewing_keys_for_card = CARD_VIEWING_KEY
        .add_suffix(info.sender.as_bytes())
        .add_suffix(&[index]);

    //viewing key as bytes
    let viewing_key = new_viewing_key(&env, info, ENTROPY.load(deps.storage)?.as_bytes());

    //add viewing key to viewing key map
    viewing_keys_for_card.insert(deps.storage, &viewing_key, &true)?;

    let res = Response::default().add_attribute("viewing_key", viewing_key);

    Ok(res)
}

pub fn new_viewing_key(env: &Env, info: MessageInfo, entropy_bytes: &[u8]) -> String {
    let entropy_len = 16 + info.sender.as_bytes().len() + entropy_bytes.len();
    let mut rng_entropy = Vec::with_capacity(entropy_len);
    rng_entropy.extend_from_slice(&env.block.time.nanos().to_be_bytes());
    rng_entropy.extend_from_slice(info.sender.as_bytes());
    rng_entropy.extend_from_slice(entropy_bytes);
    let mut rng = Prng::new(entropy_bytes, &rng_entropy);
    let rand_slice = rng.rand_bytes();
    let key = sha_256(&rand_slice);

    base64::encode(&key)
}

pub fn try_create_card(
    deps: DepsMut,
    info: MessageInfo,
    card: Card,
    index: u8,
) -> StdResult<Response> {
    //add_suffix needs byte array, this is called pre-fixing
    USER_CARDS
        .add_suffix(info.sender.as_bytes())
        .insert(deps.storage, &index, &card)?;

    Ok(Response::default())
}

pub fn try_burn_card(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    index: u8,
) -> StdResult<Response> {
    let user_exists = USER_CARDS
        .add_suffix(info.sender.as_bytes())
        .get(deps.storage, &index);
    match user_exists {
        Some(_) => USER_CARDS
            .add_suffix(info.sender.as_bytes())
            .remove(deps.storage, &index)?,
        None => {}
    }

    Ok(Response::default())
}

// pub fn try_scan_viewing_key(
//     deps: DepsMut,
//     info: MessageInfo,
//     wallet: Addr,
//     index: u8,
//     viewing_key: String,
// ) -> StdResult<Response> {
//     //map for viewing keys: specifies for each wallet at a specific index
//     let viewing_keys_for_card = SCANNED_VIEWING_KEYS.add_suffix(&info.sender.as_bytes());

//     //add viewing key to viewing key map
//     viewing_keys_for_card.push_front(
//         deps.storage,
//         &ScannedViewingKey {
//             wallet: wallet,
//             index: index,
//             viewing_key: viewing_key.clone(),
//         },
//     )?;

//     let res = Response::default().add_attribute("viewing_key", viewing_key);

//     Ok(res)
// }

// pub fn query_all_scanned_viewing_keys(
//     deps: Deps,
//     env: Env,
//     permit: Permit,
// ) -> StdResult<ViewingKeysResponse> {
//     let addr = env.contract.address;

//     //check if permit is valid.  Otherwise, return error
//     let wallet_address = validate(deps, "invalid_permits", &permit, addr.to_string(), None)?;

//     let mut viewing_keys = vec![];

//     let scanned_viewing_keys_for_wallet =
//         SCANNED_VIEWING_KEYS.add_suffix(&wallet_address.as_bytes());
//     let scanned_viewing_keys_for_wallet_iter =
//         scanned_viewing_keys_for_wallet.iter(deps.storage)?;

//     for vk in scanned_viewing_keys_for_wallet_iter {
//         if vk.is_err() {
//             continue;
//         }

//         viewing_keys.push(vk.unwrap());
//     }

//     Ok(ViewingKeysResponse {
//         viewing_keys: viewing_keys,
//     })
// }

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCard {
            wallet,
            viewing_key,
            index,
        } => to_binary(&query_card(deps, wallet, viewing_key, index)?),
        // QueryMsg::GetAllViewingKeys {
        //     viewing_keys_permit,
        // } => to_binary(&query_all_scanned_viewing_keys(
        //     deps,
        //     env,
        //     viewing_keys_permit,
        // )?),
    }
}

fn query_card(deps: Deps, wallet: Addr, viewing_key: String, index: u8) -> StdResult<CardResponse> {
    //update query function to only work if you pass in a valid viewing key
    let viewing_keys_exists = CARD_VIEWING_KEY
        .add_suffix(wallet.as_bytes())
        .add_suffix(&[index]);

    if viewing_keys_exists.contains(deps.storage, &viewing_key) {
        let card_exists = USER_CARDS
            .add_suffix(wallet.as_bytes())
            .get(deps.storage, &index);

        match card_exists {
            Some(card) => Ok(CardResponse { card: card }),
            None => Err(StdError::generic_err("Card doesn't exist")),
        }
    } else {
        Err(StdError::generic_err(
            "You don't have the correct viewing key!",
        ))
    }
}
//https://docs.scrt.network/secret-network-documentation/development/snips/snip-24-query-permits-for-snip-20-tokens#data-structures
