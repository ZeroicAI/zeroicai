//! Messaging example - Agents communicating with each other

use zeroicai::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📨 ZeroicAI Messaging Example\n");

    // Create two agents
    let agent1_id = AgentId::new();
    let agent2_id = AgentId::new();
    
    println!("✅ Created agents:");
    println!("   Agent 1: {}", agent1_id);
    println!("   Agent 2: {}", agent2_id);

    // Create message router
    let mut router = Router::new();
    router.register(agent1_id.clone());
    router.register(agent2_id.clone());
    
    println!("\n✅ Router initialized with {} agents", router.agent_count());

    // Create and send a message
    let message = Message::new(
        agent1_id.clone(),
        agent2_id.clone(),
        Performative::Inform,
        "Hello from Agent 1!",
    );
    
    println!("\n📤 Sending message:");
    println!("   From: {}", message.sender());
    println!("   To: {}", message.receiver());
    println!("   Type: {:?}", message.performative());
    println!("   Content: {}", message.content());
    
    router.route(message).await?;
    
    println!("\n✅ Message delivered!");
    println!("🎉 Messaging example complete!");
    
    Ok(())
}
