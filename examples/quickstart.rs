//! Quickstart example - Demonstrating the facade works

use zeroicai::prelude::*;

fn main() {
    println!("🤖 ZeroicAI Quickstart Example\n");

    // Actually use types from each crate to prove the facade works
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    println!("z-core: Created agents {} and {}", agent1, agent2);

    let message = Message::new(agent1, agent2, Performative::Inform, "Hello!");
    println!("z-messaging: Created message: {}", message.content());

    let agent = BDIAgent::new(AgentId::new());
    println!("z-cognition: BDI agent has {} beliefs", agent.belief_count());

    let swarm = SwarmStructure::new("TestSwarm");
    println!("z-patterns: Swarm '{}' created", swarm.name());

    let _config = RuntimeConfig::default();
    println!("z-runtime: RuntimeConfig created");

    println!("\n All 5 crates working through the facade!");
}