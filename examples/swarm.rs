//! Swarm example - Collective intelligence with multiple agents

use zeroicai::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("ZeroicAI Swarm Example\n");

    // Create a swarm structure
    let mut swarm = SwarmStructure::new("ExplorationSwarm");

    // Add agents to swarm
    for i in 0..5 {
        let id = AgentId::new();
        swarm.add_member(id.clone());
        println!(" Added agent {}: {}", i + 1, id);
    }

    println!("\n Swarm '{}' has {} members", swarm.name(), swarm.size());

    // Set swarm behavior (flocking)
    let behavior = Behavior::new(BehaviorType::Flocking);
    swarm.set_behavior(behavior);
    println!(" Behavior set: {:?}", swarm.behavior().unwrap().behavior_type());

    // Run a consensus vote among swarm members
    let mut consensus = ConsensusProtocol::new(0.6);
    for (i, member) in swarm.members().iter().enumerate() {
        let choice = if i < 3 { "explore_north" } else { "explore_south" };
        consensus.vote(member.clone(), choice);
        println!("   Agent {} voted: {}", i + 1, choice);
    }

    if consensus.is_reached() {
        println!("\n Consensus reached: {}", consensus.winner().unwrap());
    } else {
        println!("\n No consensus reached");
    }

    println!("\n Swarm example complete!");

    Ok(())
}