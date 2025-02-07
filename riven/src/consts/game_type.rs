#![cfg_attr(any(), rustfmt::skip)]
///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use serde::{ Serialize, Deserialize };
use strum_macros::{ EnumString, Display, AsRefStr, IntoStaticStr };

/// League of Legends game type: matched game, custom game, or tutorial game.
#[derive(Debug, Copy, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, Display, AsRefStr, IntoStaticStr)]
#[derive(Serialize, Deserialize)]
#[repr(u8)]
pub enum GameType {
    /// Custom games
    #[strum(to_string = "CUSTOM_GAME", serialize = "CUSTOM")]
    #[serde(alias = "CUSTOM")]
    CUSTOM_GAME,
    /// all other games
    #[strum(to_string = "MATCHED_GAME", serialize = "MATCHED")]
    #[serde(alias = "MATCHED")]
    MATCHED_GAME,
    /// Tutorial games
    #[strum(to_string = "TUTORIAL_GAME", serialize = "TUTORIAL")]
    #[serde(alias = "TUTORIAL")]
    TUTORIAL_GAME,
}

#[cfg(test)]
mod test;
