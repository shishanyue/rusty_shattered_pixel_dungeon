
use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct TerrainFlags: u8 {
        const PASSABLE       = 0b00000001;
        const LOS_BLOCKING  = 0b00000010;
        const FLAMABLE      = 0b00000100;
        const SECRET        = 0b00001000;
        const SOLID         = 0b00010000;
        const AVOID         = 0b00100000;
        const LIQUID        = 0b01000000;
        const PIT           = 0b10000000;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Terrain {
    Chasm = 0,
    Empty = 1,
    Grass = 2,
    EmptyWell = 3,
    Wall = 4,
    Door = 5,
    OpenDoor = 6,
    Entrance = 7,
    EntranceSp = 37,
    Exit = 8,
    Embers = 9,
    LockedDoor = 10,
    CrystalDoor = 31,
    Pedestal = 11,
    WallDeco = 12,
    Barricade = 13,
    EmptySp = 14,
    HighGrass = 15,
    FurrowedGrass = 30,
    SecretDoor = 16,
    SecretTrap = 17,
    Trap = 18,
    InactiveTrap = 19,
    EmptyDeco = 20,
    LockedExit = 21,
    UnlockedExit = 22,
    CustomDeco = 23,
    Well = 24,
    Statue = 25,
    StatueSp = 26,
    Bookshelf = 27,
    Alchemy = 28,
    Water = 29,
    CustomDecoEmpty = 32,
    RegionDeco = 33,
    RegionDecoAlt = 34,
    MineCrystal = 35,
    MineBoulder = 36,
}

impl Terrain {
    pub fn flags(self) -> TerrainFlags {
        match self {
            Terrain::Chasm => TerrainFlags::AVOID | TerrainFlags::PIT,
            Terrain::Empty => TerrainFlags::PASSABLE,
            Terrain::Grass => TerrainFlags::PASSABLE | TerrainFlags::FLAMABLE,
            Terrain::EmptyWell => TerrainFlags::PASSABLE,
            Terrain::Water => TerrainFlags::PASSABLE | TerrainFlags::LIQUID,
            Terrain::Wall => TerrainFlags::LOS_BLOCKING | TerrainFlags::SOLID,
            Terrain::Door => TerrainFlags::PASSABLE | TerrainFlags::LOS_BLOCKING | TerrainFlags::FLAMABLE | TerrainFlags::SOLID,
            Terrain::OpenDoor => TerrainFlags::PASSABLE | TerrainFlags::FLAMABLE,
            Terrain::Entrance | Terrain::EntranceSp => TerrainFlags::PASSABLE,
            Terrain::Exit => TerrainFlags::PASSABLE,
            Terrain::Embers => TerrainFlags::PASSABLE,
            Terrain::LockedDoor => TerrainFlags::LOS_BLOCKING | TerrainFlags::SOLID,
            Terrain::CrystalDoor => TerrainFlags::SOLID,
            Terrain::Pedestal => TerrainFlags::PASSABLE,
            Terrain::WallDeco => TerrainFlags::LOS_BLOCKING | TerrainFlags::SOLID,
            Terrain::Barricade => TerrainFlags::FLAMABLE | TerrainFlags::SOLID | TerrainFlags::LOS_BLOCKING,
            Terrain::EmptySp => TerrainFlags::PASSABLE,
            Terrain::HighGrass | Terrain::FurrowedGrass => TerrainFlags::PASSABLE | TerrainFlags::LOS_BLOCKING | TerrainFlags::FLAMABLE,
            Terrain::SecretDoor => TerrainFlags::LOS_BLOCKING | TerrainFlags::SOLID | TerrainFlags::SECRET,
            Terrain::SecretTrap => TerrainFlags::PASSABLE | TerrainFlags::SECRET,
            Terrain::Trap => TerrainFlags::AVOID,
            Terrain::InactiveTrap => TerrainFlags::PASSABLE,
            Terrain::EmptyDeco | Terrain::CustomDecoEmpty => TerrainFlags::PASSABLE,
            Terrain::LockedExit => TerrainFlags::SOLID,
            Terrain::UnlockedExit => TerrainFlags::PASSABLE,
            Terrain::Well => TerrainFlags::AVOID,
            Terrain::Bookshelf => TerrainFlags::FLAMABLE | TerrainFlags::SOLID | TerrainFlags::LOS_BLOCKING,
            Terrain::Alchemy => TerrainFlags::SOLID,
            Terrain::CustomDeco | Terrain::Statue | Terrain::StatueSp | 
            Terrain::RegionDeco | Terrain::RegionDecoAlt | 
            Terrain::MineCrystal | Terrain::MineBoulder => TerrainFlags::SOLID,
        }
    }

    pub fn discover(terr: u8) -> u8 {
        match terr {
            x if x == Terrain::SecretDoor as u8 => Terrain::Door as u8,
            x if x == Terrain::SecretTrap as u8 => Terrain::Trap as u8,
            _ => terr,
        }
    }
}

// 提供从整数值到Terrain枚举的转换
impl TryFrom<u8> for Terrain {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Terrain::Chasm),
            1 => Ok(Terrain::Empty),
            2 => Ok(Terrain::Grass),
            3 => Ok(Terrain::EmptyWell),
            4 => Ok(Terrain::Wall),
            5 => Ok(Terrain::Door),
            6 => Ok(Terrain::OpenDoor),
            7 => Ok(Terrain::Entrance),
            37 => Ok(Terrain::EntranceSp),
            8 => Ok(Terrain::Exit),
            9 => Ok(Terrain::Embers),
            10 => Ok(Terrain::LockedDoor),
            31 => Ok(Terrain::CrystalDoor),
            11 => Ok(Terrain::Pedestal),
            12 => Ok(Terrain::WallDeco),
            13 => Ok(Terrain::Barricade),
            14 => Ok(Terrain::EmptySp),
            15 => Ok(Terrain::HighGrass),
            30 => Ok(Terrain::FurrowedGrass),
            16 => Ok(Terrain::SecretDoor),
            17 => Ok(Terrain::SecretTrap),
            18 => Ok(Terrain::Trap),
            19 => Ok(Terrain::InactiveTrap),
            20 => Ok(Terrain::EmptyDeco),
            21 => Ok(Terrain::LockedExit),
            22 => Ok(Terrain::UnlockedExit),
            23 => Ok(Terrain::CustomDeco),
            24 => Ok(Terrain::Well),
            25 => Ok(Terrain::Statue),
            26 => Ok(Terrain::StatueSp),
            27 => Ok(Terrain::Bookshelf),
            28 => Ok(Terrain::Alchemy),
            29 => Ok(Terrain::Water),
            32 => Ok(Terrain::CustomDecoEmpty),
            33 => Ok(Terrain::RegionDeco),
            34 => Ok(Terrain::RegionDecoAlt),
            35 => Ok(Terrain::MineCrystal),
            36 => Ok(Terrain::MineBoulder),
            _ => Err(()),
        }
    }
}