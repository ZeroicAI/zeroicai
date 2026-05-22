//! Integration tests for the facade crate

use zeroicai::prelude::*;

#[test]
fn test_prelude_imports() {
    // Core
    let _agent_id = AgentId::new();
    
    // Messaging
    let sender = AgentId::new();
    let receiver = AgentId::new();
    let _message = Message::new(sender, receiver, Performative::Inform, "test");
    
    // Runtime
    let _config = RuntimeConfig::default();
}

#[tokio::test]
async fn test_runtime_creation() {
    let config = RuntimeConfig::default();
    let runtime = Runtime::new(config);
    assert!(runtime.is_ok());
}

#[tokio::test]
async fn test_messaging_integration() {
    let agent1 = AgentId::new();
    let agent2 = AgentId::new();
    
    let mut router = Router::new();
    router.register(agent1.clone());
    router.register(agent2.clone());
    
    let message = Message::new(agent1, agent2, Performative::Request, "test");
    let result = router.route(message).await;
    
    assert!(result.is_ok());
}

#[test]
fn test_bdi_agent_creation() {
    let agent = BDIAgent::new(AgentId::new());
    assert_eq!(agent.belief_count(), 0);
    assert_eq!(agent.desire_count(), 0);
}

#[test]
fn test_swarm_structure() {
    let swarm = SwarmStructure::new("TestSwarm");
    assert_eq!(swarm.name(), "TestSwarm");
}
