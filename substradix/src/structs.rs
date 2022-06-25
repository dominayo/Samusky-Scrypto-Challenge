use scrypto::prelude::*;

#[derive(TypeId,  Describe, Clone, PartialEq, Decode, Encode)]
pub struct Item {
    pub id: Decimal,
    pub version: Decimal,
    pub weight: Decimal,
    pub level: u64, 
}

#[derive(NonFungibleData, TypeId, Decode, Encode, Describe, Clone, PartialEq)]
pub struct Receipt {
    #[scrypto(mutable)]
    pub price: Decimal,
    pub id: NonFungibleId, 
    pub category: Categories,
    pub item_id: NonFungibleId
}

#[derive(NonFungibleData, TypeId, Decode, Encode, Describe, Clone, PartialEq)]
pub struct Character {
    #[scrypto(mutable)]
    pub name: String,
    #[scrypto(mutable)]
    pub class: u64,
    #[scrypto(mutable)]
    pub level: u64,
    #[scrypto(mutable)]
    pub exp: u128,
    #[scrypto(mutable)]
    pub stage: Decimal, 
    #[scrypto(mutable)]
    pub health: Decimal,
    #[scrypto(mutable)]
    pub attack: Decimal,
    #[scrypto(mutable)]
    pub magic: Decimal,
    #[scrypto(mutable)]
    pub defense: Decimal,
    #[scrypto(mutable)]
    pub speed: Decimal,
    #[scrypto(mutable)]
    pub version: Decimal,
}

#[derive(NonFungibleData, TypeId, Decode, Encode, Describe, Clone, PartialEq)]
pub struct Weapon {
    #[scrypto(mutable)]
    pub class: u64,
    #[scrypto(mutable)]
    pub item_info: Item,
    #[scrypto(mutable)]
    pub physical_base: Decimal,
    #[scrypto(mutable)]
    pub physical_scaling: Decimal,
    #[scrypto(mutable)]
    pub spell_base: Decimal,
    #[scrypto(mutable)]
    pub spell_scaling: Decimal,
    #[scrypto(mutable)]
    pub ability: Decimal,
    #[scrypto(mutable)]
    pub ability_odds: Decimal,
    #[scrypto(mutable)]
    pub range: Decimal,
    
}

impl Weapon {
    pub fn null_weapon() -> Self {
        let null_item = Item {
            id: dec!(0),
            version: dec!(1),
            weight: dec!(0),
            level: 0,
        };
        Self { 
            class: 0,
            item_info: null_item,
            physical_base: dec!(0),
            physical_scaling: dec!(".5"),
            spell_base: dec!(0),
            spell_scaling: dec!(".5"),
            ability_odds: dec!(0),
            ability: dec!(0),
            range: dec!(1), 
        }
    }
}

#[derive(NonFungibleData, TypeId, Decode, Encode, Describe, Clone, PartialEq)]
pub struct Armor {
    #[scrypto(mutable)]
    pub item_info: Item,
    #[scrypto(mutable)]
    pub part: ArmorNames,
    #[scrypto(mutable)]
    pub health: Decimal,
    #[scrypto(mutable)]
    pub defense: Decimal,
}

impl Armor {
    pub fn null_armor(name: ArmorNames) -> Self {
        let null_item = Item {
            id: dec!(0),
            version: dec!(1),
            weight: dec!(0),
            level: 0,
        };
        Self {  part: name, health: dec!(0), defense: dec!(1), item_info: null_item}
    }
}

#[derive(NonFungibleData, TypeId, Decode, Encode, Describe, Clone, PartialEq)]
pub struct Accessory {
    #[scrypto(mutable)]
    pub item_info: Item,
    #[scrypto(mutable)]
    pub part: AccessoryNames,
    #[scrypto(mutable)]
    pub attack: Decimal,
    #[scrypto(mutable)]
    pub magic: Decimal,
    #[scrypto(mutable)]
    pub speed: Decimal,
    #[scrypto(mutable)]
    pub ability: Decimal, 
    #[scrypto(mutable)]
    pub ability_odds: Decimal,
}

impl Accessory {
    pub fn null_accessory(name: AccessoryNames) -> Self {
        let null_item = Item {
            id: dec!(0),
            version: dec!(1),
            weight: dec!(0),
            level: 0,
        };
        Self { part: name, attack: dec!(1), magic: dec!(1), speed: dec!(0), 
            item_info: null_item, ability: dec!(0), ability_odds: dec!(0)}
    }
}

#[derive(TypeId, Decode, Encode, Describe, Copy, Clone)]
pub struct CombatInfo {
    pub health: Decimal,
    pub damage: Decimal,
    pub defense: Decimal,
    pub speed: Decimal,
    // Decimals go <(Ability #, Ability odds, Ability damage)>
    pub ability_weapon: Option<(Decimal,Decimal,Decimal)>,
    pub ability_gloves: Option<(Decimal,Decimal,Decimal)>,
    pub ability_belt: Option<(Decimal,Decimal,Decimal)>,
    pub ability_shoes: Option<(Decimal,Decimal,Decimal)>,
}

#[derive(TypeId, Decode, Encode, Describe, Copy, Clone)]
pub struct EnemyFullInfo {
    pub combat_info: CombatInfo,
    pub gold_on_loss: u64,
    pub greavite_on_loss: u64,
    pub wood_on_loss: u64,
    pub gold_on_win: u64,
    pub greavite_on_win: u64,
    pub wood_on_win: u64,
    pub exp_on_loss: u128,
    pub exp_on_win: u128,
}
#[derive(TypeId, Decode, Encode, Describe, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Stats {
    Health,
    Attack,
    Magic,
    Defense,
    Speed,
}
#[derive(TypeId, Decode, Encode, Describe, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ArmorNames {
    Helmet,
    Chest,
    Pants,
}
#[derive(TypeId, Decode, Encode, Describe, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AccessoryNames {
    Gloves,
    Belt,
    Shoes,
}

#[derive(TypeId, Decode, Encode, Describe, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Categories {
    Weapon,
    Armor,
    Accessory,
}

#[derive(TypeId, Decode, Encode, Describe, Clone)]
pub struct SharedData {
    pub gold_cost: Decimal,
    pub resource_1: ResourceAddress,
    pub resource_2: ResourceAddress,
    pub resource_1_cost: Decimal,
    pub resource_2_cost: Decimal,
    pub weight: Decimal,
}

#[derive(TypeId, Decode, Encode, Describe, Clone)]
pub struct WeaponData {
    pub crafting_data: SharedData,
    pub min_rng_roll: u128,
    pub max_rng_roll: u128,
    pub success_number: u128,
    pub ability_if_success: Decimal,
    pub ability_active_odds: Decimal,
    pub physical_base: Decimal,
    pub physical_scaling: Decimal,
    pub magic_base: Decimal,
    pub magic_scaling: Decimal,
    pub range: Decimal,
}

#[derive(TypeId, Decode, Encode, Describe, Clone)]
pub struct ArmorData {
    pub crafting_data: SharedData,
    pub health: Decimal,
    pub health_bonus: Decimal,
    pub defense: Decimal,
    pub defense_bonus: Decimal,
}

#[derive(TypeId, Decode, Encode, Describe, Clone)]
pub struct AccessoryData {
    pub crafting_data: SharedData,
    pub min_rng_roll: u128,
    pub max_rng_roll: u128,
    pub success_number: u128,
    pub ability_if_success: Decimal,
    pub ability_active_odds: Decimal,
    pub attack: Decimal,
    pub attack_bonus: Decimal,
    pub magic: Decimal,
    pub magic_bonus: Decimal,
    pub speed: Decimal,
    pub speed_bonus: Decimal,
}

#[derive(TypeId, Decode, Encode, Describe, Clone)]
pub struct GameData {
    pub game_version:  Decimal,
    pub game_price: Decimal,
    pub character_number: u64,
    pub char_hp: HashMap<u64, Vec<u64>>,
    pub char_atk: HashMap<u64, Vec<u64>>,
    pub char_mag: HashMap<u64, Vec<u64>>,
    pub char_def: HashMap<u64, Vec<u64>>,
    pub char_spd: HashMap<u64, Vec<u64>>,
    pub stage_data: HashMap<u64, Vec<EnemyFullInfo>>,
    pub exp_data: Vec<u128>,
    pub weapon_data: HashMap<(u64, Decimal), WeaponData>,
    pub armor_data: HashMap<(ArmorNames, Decimal), ArmorData>,
    pub accessory_data: HashMap<(AccessoryNames, Decimal), AccessoryData>,
}