#![cfg_attr(rustfmt, rustfmt_skip)]
///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

use strum_macros::{ EnumString, EnumVariantNames, IntoStaticStr };

/// LoL or TFT ranked queue types.
#[non_exhaustive]
#[derive(Debug, Clone)]
#[derive(Eq, PartialEq, Hash)]
#[derive(EnumString, EnumVariantNames, IntoStaticStr)]
#[repr(u8)]
pub enum QueueType {
    /// Catch-all variant for new, unknown queue types.
    #[strum(default)]
    UNKNOWN(String),

    /// 5v5 Ranked Solo games
    RANKED_SOLO_5x5,
    /// 5v5 Ranked Flex games
    RANKED_FLEX_SR,
    /// 3v3 Ranked Flex games
    /// Deprecated in patch 9.23
    #[deprecated(note="Deprecated in patch 9.23")]
    RANKED_FLEX_TT,
    /// Ranked Teamfight Tactics games
    RANKED_TFT,
    /// Ranked Teamfight Tactics (Hyper Roll) games
    RANKED_TFT_TURBO,
    /// Ranked Teamfight Tactics (Double Up Workshop) games
    /// Deprecated in patch 12.11 in favor of queueId 1160 (`RANKED_TFT_DOUBLE_UP`)
    #[deprecated(note="Deprecated in patch 12.11 in favor of queueId 1160 (`RANKED_TFT_DOUBLE_UP`)")]
    RANKED_TFT_PAIRS,
    /// Ranked Teamfight Tactics (Double Up Workshop) games
    RANKED_TFT_DOUBLE_UP,
    /// "Arena" games
    CHERRY,
}

serde_strum_unknown!(QueueType);

#[cfg(test)]
mod test;
