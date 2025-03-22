export const idlFactory = ({ IDL }) => {
  const Role = IDL.Variant({ 'user' : IDL.Null, 'system' : IDL.Null });
  const ChatMessage = IDL.Record({ 'content' : IDL.Text, 'role' : Role });
  return IDL.Service({
    'chat' : IDL.Func([IDL.Vec(ChatMessage)], [IDL.Text], []),
    'prompt' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
