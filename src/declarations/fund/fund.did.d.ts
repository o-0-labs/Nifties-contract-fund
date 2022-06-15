import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CrowdArgs {
  'status' : number,
  'funded_total_ammount' : bigint,
  'owner' : string,
  'name' : string,
  'end_time' : bigint,
  'begin_time' : bigint,
  'total_ammount' : bigint,
  'min_ammount' : bigint,
}
export type CrowdResult = { 'Ok' : number } |
  { 'Err' : string };
export interface Funder {
  'fund_total_ammount' : bigint,
  'funder_records' : Array<FunderRecord>,
  'funder' : string,
  'last_fund_time' : bigint,
}
export interface FunderRecord { 'fund_time' : bigint, 'fund_ammount' : bigint }
export type Memo = bigint;
export interface Tokens { 'e8s' : bigint }
export type TransferResult = { 'Ok' : Memo } |
  { 'Err' : string };
export interface _SERVICE {
  'create_crowd' : ActorMethod<[CrowdArgs], CrowdResult>,
  'fund' : ActorMethod<[bigint], number>,
  'get_crowd' : ActorMethod<[], CrowdArgs>,
  'get_funder' : ActorMethod<[string], [] | [Funder]>,
  'get_funders' : ActorMethod<[], Array<Funder>>,
  'greet' : ActorMethod<[string], string>,
  'token_transfer' : ActorMethod<[], string>,
}
