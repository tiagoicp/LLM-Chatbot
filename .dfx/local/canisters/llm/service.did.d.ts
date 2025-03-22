import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface chat_message { 'content' : string, 'role' : role }
export interface chat_request {
  'model' : string,
  'messages' : Array<chat_message>,
}
export type role = { 'user' : null } |
  { 'assistant' : null } |
  { 'system' : null };
export interface _SERVICE { 'v0_chat' : ActorMethod<[chat_request], string> }
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
