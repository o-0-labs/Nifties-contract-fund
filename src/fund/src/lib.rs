
use std::borrow::BorrowMut;
use std::{cell::RefCell, str::FromStr};
use std::hash::Hash;
use candid::{candid_method, CandidType, Principal,Nat};

use ic_cdk_macros::*;
use std::collections::BTreeMap;
use ic_ledger_types::{AccountIdentifier, TransferResult, BlockIndex,DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, Memo, Subaccount,Tokens};
use ic_cdk::api::{caller,call::CallResult};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use parking_lot::{MappedRwLockReadGuard, RwLock, RwLockReadGuard};
mod utils;
 
use std::fmt;
// use candid::{candid_method, CandidType, Deserialize, Int, Nat};
// use serde::{Deserialize, Serialize};
type Funders = BTreeMap<String, Funder>;
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct CrowdFund{
    pub name:String,
    pub begin_time:u64,
    pub end_time:u64,
    pub owner:String, //
    pub min_ammount:u64,
    pub total_ammount:u64,
    pub funded_total_ammount:u64,
    pub token_id:String,
    pub funders: Funders,
    pub status:u8,
}
impl Funder{
    pub fn add(&mut self ,ammount:u64){
        let fund_time = ic_cdk::api::time();
        let funder = FunderRecord{
            fund_ammount:ammount,
            fund_time:fund_time
        };
        self.fund_total_ammount += ammount;
        self .last_fund_time = fund_time;
        let  funders = & mut self.funder_records;
        funders.push(funder);
    }
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq,Default)]
pub struct Funder{
     pub funder:String,
     pub fund_total_ammount:u64,
     pub last_fund_time:u64,
     pub funder_records:Vec<FunderRecord>,
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq,Default)]
pub struct FunderRecord{
     pub fund_ammount:u64,
     pub fund_time:u64,
}
impl Default for CrowdFund{
    fn default() -> Self {
        CrowdFund {
            name:"".to_string(),
            begin_time:0,
            end_time:0,
            owner:"".to_string(), //
            min_ammount:0,
            total_ammount:0,
            funded_total_ammount:0,
            funders:  BTreeMap::new(),
            status:0,
            token_id:"ryjl3-tyaaa-aaaaa-aaaba-cai".to_string(),
        }
    }
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash, PartialEq)]
pub struct Conf {
    ledger_canister_id: Principal,
    contract_canister_owner:Option<Principal>
    
}

impl Default for Conf {
    fn default() -> Self {
        Conf {
            ledger_canister_id: Principal::from_str("rgy5l-a6jqi-razfu-frmjx-5akex-zuobw-k5w57-4b7w7-7vcft-byc6h-nqe").unwrap(),
            contract_canister_owner:None
          
        }
    }
}
#[derive(CandidType)]
pub enum CrowdErr {
    HasExists,
    NotAllowed,
}
thread_local! {
    static CONF: RefCell<Conf> = RefCell::new(Conf::default());
    static STORGE_CROWD: RefCell<CrowdFund> = RefCell::new(CrowdFund::default());
}
#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::print("init");
    let call = caller();
    CONF.with(|c|{
        let mut state = c.borrow_mut();
        // state.ledger_canister_id = Some(call);
        state.contract_canister_owner = Some(call);
    });
  
}
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Hash)]
pub struct CrowdArgs{
    pub name:String,
    pub begin_time:u64,
    pub end_time:u64,
    pub owner:String, //
    pub min_ammount:u64,
    pub total_ammount:u64,
    pub funded_total_ammount:u64,
    pub status:u8,
}
 
#[update]
#[candid_method(update)]
fn  create_crowd(args:CrowdArgs)->Result<u8,String>{
    STORGE_CROWD.with(|c|{
        let mut state = c.borrow_mut();
        let status = state.status;
        if status >0{ //has init
            return Err("crowd has init".to_string());
        }else{
            state.name = args.name;
            state.begin_time = args.begin_time;
            state.end_time = args.end_time;
            state.owner = args.owner;
            state.min_ammount = args.min_ammount; //-1 no limit
            state.total_ammount = args.total_ammount;
            state.funded_total_ammount = 0;
            state.status = 1;
            return Ok(0);
        }
    })
}
#[update]
#[candid_method(update)]
fn  stop_crowd()->Result<u8,CrowdErr>{
    let call = caller();
    let manage = CONF.with(|s|s.borrow().contract_canister_owner.unwrap());
    if call == manage{
        STORGE_CROWD.with(|s|s.borrow_mut().status=2);
        return Ok(0);
    }else{
        return Err(CrowdErr::NotAllowed);
    }
}

#[query]
#[candid_method(query)]
fn  get_crowd()->CrowdArgs{
     STORGE_CROWD.with(|c|{
         let state = c.borrow();
         CrowdArgs{
            name:state.name.clone(),
            begin_time : state.begin_time,
            end_time :state.end_time,
            owner :state.owner.clone(),
            min_ammount :state.min_ammount,//-1 no limit
            total_ammount : state.total_ammount,
            funded_total_ammount : state.funded_total_ammount,
            status :state.status,
         }
     })
}
#[query]
#[candid_method(query)]
fn  get_funder(funder:String)->Option<Funder>{
     STORGE_CROWD.with(|c|{
         let state = c.borrow();
        //  let funders = state.funders.clone();
         state.funders.get(&funder).cloned()
         }
        )
}
#[query]
#[candid_method(query)]
fn  get_funders()->Vec<Funder>{
     STORGE_CROWD.with(|c|{
         let state = c.borrow();
        //  let funders = state.funders.clone();
         state.funders.values().cloned().collect()
         }
        )
}
#[update]
#[candid_method(update)]
fn  fund(ammount:u64)->u8{
    let funder = caller();
    let funder_str = funder.to_string();
    ic_cdk::println!("funder:{}",&funder_str);
     STORGE_CROWD.with(|c|{
         let mut  state = c.borrow_mut();
         if state.status ==1{
            state.funded_total_ammount += ammount;
            if state.funded_total_ammount  > state.total_ammount{
                state.status =3;
            }
            let funder_records = state.funders.get(&funder_str);
            let fund_time = ic_cdk::api::time();
             if let Some(f) = funder_records{
                // pub struct Funder{
                //     pub funder:String,
                //     pub fund_total_ammount:u64,
                //     pub last_fund_time:u64,
                //     pub funder_records:Vec<FunderRecord>,
                    let mut fund  = f.clone();
                    fund.fund_total_ammount +=ammount;
                    fund.last_fund_time = fund_time;
                    let fund_record = FunderRecord{
                          fund_ammount:ammount,
                         fund_time:fund_time,
                    };
                    fund.funder_records.push(fund_record);
                    state.funders.insert(funder_str.clone(),fund);
                    1
             }else{
                let fund_record = FunderRecord{
                    fund_ammount:ammount,
                   fund_time:fund_time,
                 };
                 let mut funder_records = vec![];
                 funder_records.push(fund_record);
                let funder = Funder{
                    funder :funder_str.clone(),
                    fund_total_ammount:ammount,
                    last_fund_time:fund_time,
                    funder_records:funder_records
                };
                state.funders.insert(funder_str.clone(),funder);
                2
             }
         }else{
            state.status
         }
         }
        )
    }
    // pub type TransferResult = Result<BlockIndex, TransferError>;
#[query]
async fn token_transfer()->String {

    "hello".to_string()
    // let call = ic_cdk::caller();
    // ic_cdk::print("transfer begin");
    // let canister_id = ic_cdk::api::id();
    // let receive = Principal::from_str("rgy5l-a6jqi-razfu-frmjx-5akex-zuobw-k5w57-4b7w7-7vcft-byc6h-nqe").unwrap();
    // let transfer_args =  ic_ledger_types::TransferArgs {
    //         memo: Memo(0),
    //         amount: Tokens::from_e8s(100),
    //         fee: Tokens::from_e8s(10_000),
    //         from_subaccount:  Some(DEFAULT_SUBACCOUNT),
    //         to: AccountIdentifier::new(&canister_id, &DEFAULT_SUBACCOUNT),
    //         created_at_time: None,
    //     };


    //     let wicp_canister = Principal::from_str("utozz-siaaa-aaaam-qaaxq-cai").unwrap();
    // //  ic_cdk::call(wicp_canister, "transfer",(receive,10000)).await
    // //  .map_err(|e| format!("failed to call ledger: {:?}", e))
    // //  .map_err(|e| format!("ledger transfer error {:?}", e))
    //     // Ok(result)
    // //  let transfer_args = TransferArgs{
    // //     to:receive,
    // //     value:1000
    // //  };
    //  transfer(wicp_canister, call,receive,Nat::from(1)).await
    //     .map_err(|e| format!("failed to call ledger: {:?}", e))?
    //     .map_err(|e| format!("ledger transfer error {:?}", e))
 }
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum TxError {
    InsufficientBalance,
    InsufficientAllowance,
    Unauthorized,
    LedgerTrap,
    AmountTooSmall,
    BlockUsed,
    ErrorOperationStyle,
    ErrorTo,
    Other,
}
impl fmt::Display for TxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LedgerTrap=>write!(f, "transaction fee should be "),
            _=>write!(f, "transaction fee should be ")
        }
    }
}
pub type TxReceipt = Result<Nat, TxError>;
pub async fn transfer(
    ledger_canister_id: Principal,
    from:Principal,
    to: Principal,
    value:Nat
) -> CallResult<TxReceipt> {
    let canister_id = ic_cdk::api::id();
    ic_cdk::call(ledger_canister_id, "approve", (canister_id,Nat::from(100),)).await?;
    let (result ,)= ic_cdk::call(ledger_canister_id, "transferFrom", (from,to,value,)).await?;
    Ok(result)
}

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    ic_cdk::print("greet function");
    let owner =  CONF.with(|c| c.borrow().ledger_canister_id.clone());
    format!("Hello, {}!", owner.to_text())
}
