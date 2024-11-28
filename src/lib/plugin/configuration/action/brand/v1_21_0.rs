use bevy::prelude::*;
use compact_str::CompactString;
use froglight::prelude::{
    versions::v1_21_0::{login::LoginServerboundPackets, V1_21_0},
    State, *,
};

use crate::login::LoginTask;

impl SendServerBrand for V1_21_0 {
    fn send_brand(
        _entity: Entity,
        _packet: LoginServerboundPacket,
        _channel: &LoginTask<Self>,
        _brand: &ServerBrand,
    ) {
        todo!()
    }
}
