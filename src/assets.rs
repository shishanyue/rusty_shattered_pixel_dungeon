use bevy::{platform::collections::HashMap, prelude::*};

use crate::bevy_ext::system::SystemState;

// 定义资产路径和句柄的类型别名
type AssetPath = &'static str;
type AssetHandle<T> = Handle<T>;


// 宏用于快速生成资源结构体和默认实现
macro_rules! define_asset_group {
    ($name:ident<$asset_type:ident> { $($field:ident: $path:literal),* $(,)? }) => {
        #[derive(Debug)]
        pub struct $name {
            $(pub $field: (AssetPath, Handle<$asset_type>),)*
        }

        impl Default for $name {
            fn default() -> Self {
                Self {
                    $($field: ($path, Handle::<$asset_type>::default()),)*
                }
            }
        }

        impl $name {
            pub fn load(&mut self, asset_server: &Res<AssetServer>) {
                $(self.$field.1 = asset_server.load(self.$field.0);)*
            }
        }
    };
}



// 使用宏定义各个资源组
define_asset_group!(Effects<Image> {
    effects: "effects/effects.png",
    fireball: "effects/fireball.png",
    specks: "effects/specks.png",
    spell_icons: "effects/spell_icons.png",
    text_icons: "effects/text_icons.png",
});

define_asset_group!(Environment<Image> {
    terrain_features: "environment/terrain_features.png",
    visual_grid: "environment/visual_grid.png",
    wall_blocking: "environment/wall_blocking.png",
    tiles_sewers: "environment/tiles_sewers.png",
    tiles_prison: "environment/tiles_prison.png",
    tiles_caves: "environment/tiles_caves.png",
    tiles_city: "environment/tiles_city.png",
    tiles_halls: "environment/tiles_halls.png",
    tiles_caves_crystal: "environment/tiles_caves_crystal.png",
    tiles_caves_gnoll: "environment/tiles_caves_gnoll.png",
    water_sewers: "environment/water0.png",
    water_prison: "environment/water1.png",
    water_caves: "environment/water2.png",
    water_city: "environment/water3.png",
    water_halls: "environment/water4.png",
    weak_floor: "environment/custom_tiles/weak_floor.png",
    sewer_boss: "environment/custom_tiles/sewer_boss.png",
    prison_quest: "environment/custom_tiles/prison_quest.png",
    prison_exit: "environment/custom_tiles/prison_exit.png",
    caves_quest: "environment/custom_tiles/caves_quest.png",
    caves_boss: "environment/custom_tiles/caves_boss.png",
    city_boss: "environment/custom_tiles/city_boss.png",
    halls_sp: "environment/custom_tiles/halls_special.png",
});

define_asset_group!(Sounds<AudioSource>{
    click: "sounds/click.mp3",
    badge: "sounds/badge.mp3",
    gold: "sounds/gold.mp3",
    open: "sounds/door_open.mp3",
    unlock: "sounds/unlock.mp3",
    item: "sounds/item.mp3",
    dewdrop: "sounds/dewdrop.mp3",
    step: "sounds/step.mp3",
    water: "sounds/water.mp3",
    grass: "sounds/grass.mp3",
    trample: "sounds/trample.mp3",
    sturdy: "sounds/sturdy.mp3",
    hit: "sounds/hit.mp3",
    miss: "sounds/miss.mp3",
    hit_slash: "sounds/hit_slash.mp3",
    hit_stab: "sounds/hit_stab.mp3",
    hit_crush: "sounds/hit_crush.mp3",
    hit_magic: "sounds/hit_magic.mp3",
    hit_strong: "sounds/hit_strong.mp3",
    hit_parry: "sounds/hit_parry.mp3",
    hit_arrow: "sounds/hit_arrow.mp3",
    atk_spiritbow: "sounds/atk_spiritbow.mp3",
    atk_crossbow: "sounds/atk_crossbow.mp3",
    health_warn: "sounds/health_warn.mp3",
    health_critical: "sounds/health_critical.mp3",
    descend: "sounds/descend.mp3",
    eat: "sounds/eat.mp3",
    read: "sounds/read.mp3",
    lullaby: "sounds/lullaby.mp3",
    drink: "sounds/drink.mp3",
    shatter: "sounds/shatter.mp3",
    zap: "sounds/zap.mp3",
    lightning: "sounds/lightning.mp3",
    levelup: "sounds/levelup.mp3",
    death: "sounds/death.mp3",
    challenge: "sounds/challenge.mp3",
    cursed: "sounds/cursed.mp3",
    trap: "sounds/trap.mp3",
    evoke: "sounds/evoke.mp3",
    tomb: "sounds/tomb.mp3",
    alert: "sounds/alert.mp3",
    meld: "sounds/meld.mp3",
    boss: "sounds/boss.mp3",
    blast: "sounds/blast.mp3",
    plant: "sounds/plant.mp3",
    ray: "sounds/ray.mp3",
    beacon: "sounds/beacon.mp3",
    teleport: "sounds/teleport.mp3",
    charms: "sounds/charms.mp3",
    mastery: "sounds/mastery.mp3",
    puff: "sounds/puff.mp3",
    rocks: "sounds/rocks.mp3",
    burning: "sounds/burning.mp3",
    falling: "sounds/falling.mp3",
    ghost: "sounds/ghost.mp3",
    secret: "sounds/secret.mp3",
    bones: "sounds/bones.mp3",
    bee: "sounds/bee.mp3",
    degrade: "sounds/degrade.mp3",
    mimic: "sounds/mimic.mp3",
    debuff: "sounds/debuff.mp3",
    chargeup: "sounds/chargeup.mp3",
    gas: "sounds/gas.mp3",
    chains: "sounds/chains.mp3",
    scan: "sounds/scan.mp3",
    sheep: "sounds/sheep.mp3",
    mine: "sounds/mine.mp3",
});

define_asset_group!(Splashes<Image>{
    warrior: "splashes/warrior.jpg",
    mage: "splashes/mage.jpg",
    rogue: "splashes/rogue.jpg",
    huntress: "splashes/huntress.jpg",
    duelist: "splashes/duelist.jpg",
    cleric: "splashes/cleric.jpg",
    sewers: "splashes/sewers.jpg",
    prison: "splashes/prison.jpg",
    caves: "splashes/caves.jpg",
    city: "splashes/city.jpg",
    halls: "splashes/halls.jpg",
});
define_asset_group!(Sprites<Image>{
    items: "sprites/items.png",
    item_icons: "sprites/item_icons.png",
    warrior: "sprites/warrior.png",
    mage: "sprites/mage.png",
    rogue: "sprites/rogue.png",
    huntress: "sprites/huntress.png",
    duelist: "sprites/duelist.png",
    cleric: "sprites/cleric.png",
    avatars: "sprites/avatars.png",
    pet: "sprites/pet.png",
    amulet: "sprites/amulet.png",
    rat: "sprites/rat.png",
    brute: "sprites/brute.png",
    spinner: "sprites/spinner.png",
    dm300: "sprites/dm300.png",
    wraith: "sprites/wraith.png",
    undead: "sprites/undead.png",
    king: "sprites/king.png",
    piranha: "sprites/piranha.png",
    eye: "sprites/eye.png",
    gnoll: "sprites/gnoll.png",
    crab: "sprites/crab.png",
    goo: "sprites/goo.png",
    swarm: "sprites/swarm.png",
    skeleton: "sprites/skeleton.png",
    shaman: "sprites/shaman.png",
    thief: "sprites/thief.png",
    tengu: "sprites/tengu.png",
    sheep: "sprites/sheep.png",
    keeper: "sprites/shopkeeper.png",
    bat: "sprites/bat.png",
    elemental: "sprites/elemental.png",
    monk: "sprites/monk.png",
    warlock: "sprites/warlock.png",
    golem: "sprites/golem.png",
    statue: "sprites/statue.png",
    succubus: "sprites/succubus.png",
    scorpio: "sprites/scorpio.png",
    fists: "sprites/yog_fists.png",
    yog: "sprites/yog.png",
    larva: "sprites/larva.png",
    ghost: "sprites/ghost.png",
    maker: "sprites/wandmaker.png",
    troll: "sprites/blacksmith.png",
    imp: "sprites/demon.png",
    ratking: "sprites/ratking.png",
    bee: "sprites/bee.png",
    mimic: "sprites/mimic.png",
    rot_lash: "sprites/rot_lasher.png",
    rot_heart: "sprites/rot_heart.png",
    guard: "sprites/guard.png",
    wards: "sprites/wards.png",
    guardian: "sprites/guardian.png",
    slime: "sprites/slime.png",
    snake: "sprites/snake.png",
    necro: "sprites/necromancer.png",
    ghoul: "sprites/ghoul.png",
    ripper: "sprites/ripper.png",
    spawner: "sprites/spawner.png",
    dm100: "sprites/dm100.png",
    pylon: "sprites/pylon.png",
    dm200: "sprites/dm200.png",
    lotus: "sprites/lotus.png",
    ninja_log: "sprites/ninja_log.png",
    spirit_hawk: "sprites/spirit_hawk.png",
    red_sentry: "sprites/red_sentry.png",
    crystal_wisp: "sprites/crystal_wisp.png",
    crystal_guardian: "sprites/crystal_guardian.png",
    crystal_spire: "sprites/crystal_spire.png",
    gnoll_guard: "sprites/gnoll_guard.png",
    gnoll_sapper: "sprites/gnoll_sapper.png",
    gnoll_geomancer: "sprites/gnoll_geomancer.png",
    fungal_spinner: "sprites/fungal_spinner.png",
    fungal_sentry: "sprites/fungal_sentry.png",
    fungal_core: "sprites/fungal_core.png",
});

define_asset_group!(Fonts<Font> {
    pixelfont: "fonts/pixel_font.png"
});

#[derive(Debug, Default, Resource)]
pub struct GameAssets {
    pub effects: Effects,
    pub environment: Environment,
    pub fonts: Fonts,
    pub sounds: Sounds,
    pub splashes: Splashes,
    pub sprites: Sprites,
}

pub struct AssetsHandles<T: Asset> {
    handles: HashMap<String, Handle<T>>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App,) {
        app.insert_resource(GameAssets::default()).add_systems(
            PreStartup,
            |mut game_assets: ResMut<GameAssets>, asset_server: Res<AssetServer>,mut system_state: ResMut<NextState<SystemState>>| {
                game_assets.effects.load(&asset_server);
                game_assets.environment.load(&asset_server);
                game_assets.fonts.load(&asset_server);
                game_assets.sounds.load(&asset_server);
                game_assets.splashes.load(&asset_server);
                game_assets.sprites.load(&asset_server);
                system_state.set(SystemState::Loaded);
            },
        );
    }
}
