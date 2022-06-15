import { fund } from "../../declarations/fund";
import { Principal } from '@dfinity/principal';
import nns_ledgerDid from './idls/nns_ledger.did';
// import RandomBigInt from 'random-bigint';
import { getAccountId, getTokenIdentifier } from './utils';
export const NNS_MINTING_CID = 'rkp4c-7iaaa-aaaaa-aaaca-cai';
export const NNS_LEDGER_CID = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");

  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);

  // Interact with foo actor, calling the greet method
  const greeting = await fund.greet(name);

  button.removeAttribute("disabled");

  document.getElementById("greeting").innerText = greeting;

  return false;
});
document.getElementById("transfer").addEventListener("click", async (e) => {
  
const idlFactory = ({ IDL }) => {
  const Memo = IDL.Nat64;
  const TransferResult = IDL.Variant({ 'Ok' : Memo, 'Err' : IDL.Text });
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const TransferArgs = IDL.Record({
    'memo' : Memo,
    'amount' : Tokens,
    'fee' : Tokens,
    'from_subaccount':IDL.Opt(IDL.Vec(IDL.Nat8)),
    'to':IDL.Vec(IDL.Nat8),

  });
    return IDL.Service({ 'token_transfer' : IDL.Func([], [IDL.Text], []) });
  };
  const fundFactory = ({ IDL }) => {   
      return IDL.Service({ 'fund' : IDL.Func([IDL.Nat64], [IDL.Nat8], []) });
    };
  // const idlFactory1 = ({ IDL }) => {
  //   const Memo = IDL.Nat64;
  //   const TransferResult = IDL.Variant({ 'Ok' : Memo, 'Err' : IDL.Text });
  //     return IDL.Service({ 'token_transfer' : IDL.Func([IDL.Principal], [TransferResult], []) });
  //   };
  console.log(22);
  try {
    // const helloCanisterId = 'yedcv-7qaaa-aaaan-qaitq-cai'
    const helloCanisterId = 'zamy6-vyaaa-aaaak-qapva-cai'
    const whitelist = [helloCanisterId];
 
      const publicKey = await window.ic.plug.requestConnect({whitelist,
        timeout: 50000});
      console.log(`The connected user's public key is:`, publicKey);
      // helloActor = await window.ic.plug.createActor({
      //   canisterId: helloCanisterId,
      //   interfaceFactory: idlFactory,
      // });
      const principalId = await window.ic.plug.agent.getPrincipal();
      const TRANSFER_ICP_TX = {
        idl: nns_ledgerDid,
        canisterId: NNS_LEDGER_CID,
        methodName: 'send_dfx',
        args: [{
          to: getAccountId(Principal.from('rgy5l-a6jqi-razfu-frmjx-5akex-zuobw-k5w57-4b7w7-7vcft-byc6h-nqe')),
          fee: { e8s: BigInt(10000) },
          amount: { e8s: BigInt(10000)},
          memo: 0,
          from_subaccount: [], // For now, using default subaccount to handle ICP
          created_at_time: [],
        }],
        onSuccess: async (res) => {
          console.log('transferred starverse successfully');
        },
        onFail: (res) => {
          console.log('transfer starverse error', res);
        },
      };
      const TRANSFER_XTC_TX = {
        idl: fundFactory,
        canisterId: helloCanisterId,
        methodName: 'fund',
        args: [ BigInt(10000)],
        onSuccess: async (res) => {
          console.log(res);
          console.log('transferred xtc successfully');
        },
        onFail: (res) => {
          console.log('transfer xtc error', res);
        },
      };
      console.log('Doing a bunch of transfers');
      const a = await window.ic.plug.batchTransactions([TRANSFER_ICP_TX,TRANSFER_XTC_TX])
      console.log(a);
      // console.log(greet);
     
      
   

    console.log(`Plug's user principal Id is ${principalId}`);
    // const authClient = await 
    
    // .create();
    // const identity = authClient.getIdentity();
    debugger;
  } catch (e) {
    console.log(e);
  }

  console.log(22);
 
});