//! Full stack example - Complete multi-agent system

use zeroicai::prelude::*;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 ZeroicAI Full Stack Example\n");

    // 1. Create runtime
    let config = RuntimeConfig::default();
    let runtime = Runtime::new(config)?;

    println!(" Runtime initialized");

    // 2. Create agents with BDI cognition
    let agent1 = BDIAgent::new(AgentId::new());
    let agent2 = BDIAgent::new(AgentId::new());
    
    println!("\n Created BDI agents:");
    println!("   Agent 1: {}", agent1.id());
    println!("   Agent 2: {}", agent2.id());

    // 3. Setup messaging
    let mut router = Router::new();
    router.register(agent1.id().clone());
    router.register(agent2.id().clone());
    println!("\n Message router configured");

    // 4. Create organizational structure (Hierarchy)
    let hierarchy = HierarchyStructure::new("SystemHierarchy");
    println!("\n Organizational structure: Hierarchy");

    // 5. Show system overview
    println!("\n System Overview:");
    println!("   Runtime workers: {}", runtime.worker_count());
    println!("   Active agents: 2");
    println!("   Router capacity: {}", router.agent_count());
    println!("   Organization: {}", hierarchy.name());
    
    println!("\n Full stack system ready!");
    println!("   - Agents with BDI cognition ");
    println!("   - Message passing ");
    println!("   - Organizational patterns ");
    println!("   - Async runtime ");
    
    Ok(())
}
