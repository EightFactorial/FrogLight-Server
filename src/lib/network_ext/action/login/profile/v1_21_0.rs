use froglight::{
    network::versions::v1_21_0::{login::LoginSuccessPacket, V1_21_0},
    prelude::GameProfile,
};

use crate::network::LoginTask;

impl super::LoginProfileTrait for V1_21_0 {
    fn send_profile(profile: GameProfile, task: &LoginTask<Self>) {
        task.send(LoginSuccessPacket { profile, strict_error_handling: false });
    }
}
