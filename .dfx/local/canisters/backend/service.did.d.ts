import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface ChatMessage { 'content' : string, 'role' : Role }
export type Role = { 'user' : null } |
  { 'system' : null };
export interface _SERVICE {
  'chat' : ActorMethod<[Array<ChatMessage>], string>,
  'prompt' : ActorMethod<[string], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
