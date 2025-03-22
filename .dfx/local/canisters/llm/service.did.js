export const idlFactory = ({ IDL }) => {
  const role = IDL.Variant({
    'user' : IDL.Null,
    'assistant' : IDL.Null,
    'system' : IDL.Null,
  });
  const chat_message = IDL.Record({ 'content' : IDL.Text, 'role' : role });
  const chat_request = IDL.Record({
    'model' : IDL.Text,
    'messages' : IDL.Vec(chat_message),
  });
  return IDL.Service({ 'v0_chat' : IDL.Func([chat_request], [IDL.Text], []) });
};
export const init = ({ IDL }) => { return []; };
