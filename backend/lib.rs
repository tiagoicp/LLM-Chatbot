use ic_cdk::update;
use ic_llm::{ChatMessage, Model, Role};

// Import the system prompt from the prompts module
mod prompts;
use prompts::get_system_prompt;

// Import the icp_ledger plugin from the prompts module
mod plugins;
use plugins::icp_ledger_plugin::lookup_account;

#[update]
async fn prompt(prompt_str: String) -> String {
    ic_llm::prompt(Model::Llama3_1_8B, prompt_str).await
}

#[update]
async fn chat(messages: Vec<ChatMessage>) -> String {
    // Create a system message
    let system_message = ChatMessage {
        role: Role::System,
        content: get_system_prompt(),
    };
    
    // Prepend system message to user messages
    let mut all_messages = vec![system_message];
    all_messages.extend(messages);
    
    let answer = ic_llm::chat(Model::Llama3_1_8B, all_messages).await;
    if answer.starts_with("LOOKUP(") {
        // Extract the account from LOOKUP(account)
        let account = answer
            .trim_start_matches("LOOKUP(")
            .trim_end_matches(")");
        
        lookup_account(&account).await        
    } else {
        answer
    }

}

// Export the interface for the smart contract.
ic_cdk::export_candid!();
