#![cfg_attr(any(), rustfmt::skip)]
///////////////////////////////////////////////
//                                           //
//                     !                     //
//   This file is automatically generated!   //
//           Do not directly edit!           //
//                                           //
///////////////////////////////////////////////

newtype_enum! {
    /// A League of Legends map.
    pub newtype_enum Map(u8) {
        /// `1`.
        /// Summoner's Rift
        /// Original Summer variant
        SUMMONERS_RIFT_ORIGINAL_SUMMER_VARIANT = 1,
        /// `2`.
        /// Summoner's Rift
        /// Original Autumn variant
        SUMMONERS_RIFT_ORIGINAL_AUTUMN_VARIANT = 2,
        /// `3`.
        /// The Proving Grounds
        /// Tutorial Map
        THE_PROVING_GROUNDS = 3,
        /// `4`.
        /// Twisted Treeline
        /// Original Version
        TWISTED_TREELINE_ORIGINAL_VERSION = 4,
        /// `8`.
        /// The Crystal Scar
        /// Dominion map
        THE_CRYSTAL_SCAR = 8,
        /// `10`.
        /// Twisted Treeline
        /// Last TT map
        TWISTED_TREELINE = 10,
        /// `11`.
        /// Summoner's Rift
        /// Current Version
        SUMMONERS_RIFT = 11,
        /// `12`.
        /// Howling Abyss
        /// ARAM map
        HOWLING_ABYSS = 12,
        /// `14`.
        /// Butcher's Bridge
        /// Alternate ARAM map
        BUTCHERS_BRIDGE = 14,
        /// `16`.
        /// Cosmic Ruins
        /// Dark Star: Singularity map
        COSMIC_RUINS = 16,
        /// `18`.
        /// Valoran City Park
        /// Star Guardian Invasion map
        VALORAN_CITY_PARK = 18,
        /// `19`.
        /// Substructure 43
        /// PROJECT: Hunters map
        SUBSTRUCTURE_43 = 19,
        /// `20`.
        /// Crash Site
        /// Odyssey: Extraction map
        CRASH_SITE = 20,
        /// `21`.
        /// Nexus Blitz
        /// Nexus Blitz map
        NEXUS_BLITZ = 21,
        /// `22`.
        /// Convergence
        /// Teamfight Tactics map
        CONVERGENCE = 22,
        /// `30`.
        /// Arena
        /// Map for 2v2v2v2 (`CHERRY`). Team up with a friend or venture solo in this new game mode. Face against multiple teams in chaotic battles across diverse arenas
        ARENA = 30,
        /// `33`.
        /// Swarm
        /// Map for Swarm (`STRAWBERRY`). Team up with a friend or venture solo in this horde survival mode.
        SWARM = 33,
    }
}
