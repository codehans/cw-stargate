use std::io::Cursor;

use cosmos_sdk_proto::cosmos::base::v1beta1::Coin;
use cosmos_sdk_proto::cosmos::staking::v1beta1::MsgDelegate;
use cosmos_sdk_proto::ibc::applications::transfer::v1::{
    QueryDenomTraceRequest, QueryDenomTraceResponse,
};
use cosmos_sdk_proto::traits::Message;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest, Response,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let bin = MsgDelegate {
        delegator_address: info.sender.to_string(),
        validator_address: "kujiravaloper1azdfljp04ptlazs95e5gscweavmaszw5nqgrpz".to_string(),
        amount: Some(Coin {
            denom: "ukuji".to_string(),
            amount: "1000000".to_string(),
        }),
    }
    .encode_to_vec();

    let msg = CosmosMsg::Stargate {
        type_url: "/cosmos.staking.v1beta1.MsgDelegate".to_string(),
        value: Binary::from(bin),
    };

    Ok(Response::default().add_message(msg))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, _msg: QueryMsg) -> Result<Binary, ContractError> {
    let bin = QueryDenomTraceRequest {
        hash: "295548A78785A1007F232DE286149A6FF512F180AF5657780FC89C009E2C348F".to_string(),
    }
    .encode_to_vec();

    let data = Binary::from(bin);

    let query = QueryRequest::Stargate {
        path: "/ibc.applications.transfer.v1.Query/DenomHash".to_string(),
        data,
    };

    let bin: Binary = deps.querier.query(&query)?;
    let response = QueryDenomTraceResponse::decode(&mut Cursor::new(bin.to_vec()))
        .map_err(ContractError::Decode)?;

    match response.denom_trace {
        None => Ok(to_binary("not_found")?),
        Some(trace) => Ok(to_binary(&trace.base_denom)?),
    }
}
