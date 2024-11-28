use froglight::{
    network::versions::v1_21_0::{login::LoginSuccessPacket, V1_21_0},
    prelude::GameProfile,
};

use crate::login::LoginTask;

impl super::SendProfile for V1_21_0 {
    fn send_profile(profile: &GameProfile, task: &LoginTask<Self>) {
        task.send(LoginSuccessPacket { profile: profile.clone(), strict_error_handling: false });
    }
}
