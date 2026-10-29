use azalea_client::{local_player::WorldHolder, test_utils::prelude::*};
use azalea_protocol::packets::{
    ConnectionProtocol,
    config::{ClientboundFinishConfiguration, ClientboundRegistryData},
    game::ClientboundStartConfiguration,
};
use azalea_registry::identifier::Identifier;
use simdnbt::owned::{NbtCompound, NbtTag};

fn registry(id: &str, names: &[&str]) -> ClientboundRegistryData {
    ClientboundRegistryData {
        registry_id: Identifier::new(id),
        entries: names
            .iter()
            .map(|n| (Identifier::new(*n), Some(NbtCompound::new())))
            .collect(),
    }
}

fn configure(s: &mut Simulation, banner_patterns: &[&str]) {
    s.receive_packet(ClientboundRegistryData {
        registry_id: Identifier::new("minecraft:dimension_type"),
        entries: vec![(
            Identifier::new("minecraft:overworld"),
            Some(NbtCompound::from_values(vec![
                ("height".into(), NbtTag::Int(384)),
                ("min_y".into(), NbtTag::Int(-64)),
            ])),
        )]
        .into_iter()
        .collect(),
    });
    s.receive_packet(registry("minecraft:banner_pattern", banner_patterns));
    s.tick();
    s.receive_packet(ClientboundFinishConfiguration);
    s.tick();
    s.receive_packet(default_login_packet());
    s.tick();
}

/// A proxy (like Velocity) moving the client to another server reconfigures it
/// with that server's registries; protocol ids must map to those, not to the
/// first server's entries with the new ones appended.
#[test]
fn test_reconfiguration_replaces_registries() {
    let _lock = init();

    let mut s = Simulation::new(ConnectionProtocol::Configuration);
    configure(&mut s, &["minecraft:a", "minecraft:b", "minecraft:c"]);

    s.receive_packet(ClientboundStartConfiguration);
    s.tick();
    configure(&mut s, &["minecraft:c", "minecraft:a"]);

    let id = Identifier::new("minecraft:banner_pattern");
    let keys: Vec<Option<String>> = (0..3)
        .map(|i| {
            s.component::<WorldHolder>()
                .shared
                .read()
                .registries
                .protocol_id_to_identifier(id.clone(), i)
                .map(|k| k.to_string())
        })
        .collect();
    assert_eq!(
        keys,
        vec![
            Some("minecraft:c".to_owned()),
            Some("minecraft:a".to_owned()),
            None
        ]
    );
}
