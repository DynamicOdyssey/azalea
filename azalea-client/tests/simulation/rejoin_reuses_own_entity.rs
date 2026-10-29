use azalea_client::{
    account::Account,
    join::{ConnectOpts, StartJoinServerEvent},
    test_utils::prelude::*,
};
use azalea_core::position::ChunkPos;
use azalea_entity::{LocalEntity, indexing::EntityUuidIndex};
use azalea_protocol::{
    address::{ResolvedAddr, ServerAddr},
    packets::ConnectionProtocol,
};
use azalea_registry::builtin::EntityKind;
use bevy_ecs::query::With;
use tracing::Level;
use uuid::Uuid;

/// Another client in the swarm can spawn a copy of this client's player (same
/// uuid) before this client's own login packet arrives. Despawning that copy
/// must not drop this client from the uuid index.
#[test]
fn test_despawned_copy_keeps_own_uuid_entry() {
    let _lock = init();

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    simulation
        .app
        .world_mut()
        .entity_mut(simulation.entity)
        .insert(LocalEntity);
    simulation.receive_packet(default_login_packet());
    simulation.receive_packet(make_basic_empty_chunk(ChunkPos::new(0, 0), (384 + 64) / 16));
    simulation.tick();

    // a player with our uuid (1234) under another entity id: a copy
    simulation.receive_packet(make_basic_add_entity(
        EntityKind::Player,
        456,
        (0.5, 64., 0.5),
    ));
    simulation.tick();

    // logging in again makes the copy unloaded, so it's despawned
    simulation.receive_packet(default_login_packet());
    simulation.tick();
    simulation.tick();

    let mut own = None;
    simulation.with_resource::<EntityUuidIndex>(|i| own = i.get(&Uuid::from_u128(1234)));
    assert_eq!(
        own,
        Some(simulation.entity),
        "the client lost its uuid index entry"
    );
}

/// A rejoin reuses the account's own client entity even when the uuid index
/// points somewhere else; a new entity would leave the client's components
/// (and whatever the user attached to it) behind.
#[test]
fn test_rejoin_reuses_own_entity() {
    // the connection to port 1 is refused, which may log a warning in time
    let _lock = init_with_level(Level::ERROR);

    let mut simulation = Simulation::new(ConnectionProtocol::Game);
    let account = Account::offline("azalea");
    let uuid = account.uuid();
    simulation
        .app
        .world_mut()
        .entity_mut(simulation.entity)
        .insert((LocalEntity, account.clone()));
    simulation.receive_packet(default_login_packet());
    simulation.tick();
    simulation.disconnect();
    simulation.tick();

    // the index points at something that isn't our client
    let stranger = simulation.app.world_mut().spawn_empty().id();
    simulation.with_resource_mut::<EntityUuidIndex>(|mut i| i.insert(uuid, stranger));

    simulation.write_message(StartJoinServerEvent {
        account,
        connect_opts: ConnectOpts {
            address: ResolvedAddr {
                server: ServerAddr {
                    host: "127.0.0.1".into(),
                    port: 1,
                },
                socket: "127.0.0.1:1".parse().unwrap(),
            },
            server_proxy: None,
            sessionserver_proxy: None,
        },
        start_join_callback_tx: None,
    });
    simulation.update();

    let mut own = None;
    simulation.with_resource::<EntityUuidIndex>(|i| own = i.get(&uuid));
    assert_eq!(
        own,
        Some(simulation.entity),
        "the rejoin didn't reuse the client's entity"
    );
    let mut clients = simulation
        .app
        .world_mut()
        .query_filtered::<(), With<Account>>();
    assert_eq!(
        clients.iter(simulation.app.world()).count(),
        1,
        "the rejoin made a second client"
    );
}
