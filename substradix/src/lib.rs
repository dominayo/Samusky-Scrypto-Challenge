use scrypto::prelude::*;

mod structs;
mod rng;
mod combat;

// For testing reasons, all aspects of Substradix are instantiated as a single Component. 
// In a final implementation, Gamedata would be set across multiple Components, allowing for modular updates without affecting the central blueprint.
// For example, a single component would handle combat, one for forging, one for the marketplace, and one for creating a character and character info.
blueprint! {
    struct Substradix {
        collected_xrd: Vault,
        character_nft: ResourceAddress,
        system_vault: Vault,
        developer_vault: Vault,
        gold_vault: Vault,
        weapon_nft: ResourceAddress,
        armor_nft: ResourceAddress,
        accessory_nft: ResourceAddress,
        receipt_nft: ResourceAddress,
        token_greavite: ResourceAddress,
        token_wood: ResourceAddress,
        token_gold: ResourceAddress,
        marketplace_weapon_vault: Vault,
        marketplace_accessory_vault: Vault,
        marketplace_armor_vault: Vault,
        // LazyMaps don't support Clone... or the remove function...
        marketplace_listings: LazyMap<(structs::Categories, NonFungibleId), (structs::Receipt, bool)>,
        game_data: structs::GameData,
    }

    impl Substradix {
        pub fn new(game_price: Decimal) -> (ComponentAddress, Bucket) {
            // Creates developer badge for methods. Necessary to control system_badge
            let mut developer_badge = ResourceBuilder::new_fungible()
                .metadata("name", "developer")
                .divisibility(DIVISIBILITY_NONE)
                .initial_supply(10000);
            let developer_rule: AccessRule = rule!(require(developer_badge.resource_address()));
            // Creates system badge changing NFT Data. Necessary for all game actions.
            let system_badge = ResourceBuilder::new_fungible()
                .metadata("name", "system")
                .divisibility(DIVISIBILITY_NONE)
                .mintable(developer_rule.clone(), MUTABLE(developer_rule.clone()))
                .initial_supply(1000000);
            let system_rule: AccessRule = rule!(require(system_badge.resource_address()));
            // NFTs with data
            let character_nft = ResourceBuilder::new_non_fungible()
                .metadata("type", "Substradix character NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .restrict_withdraw(AccessRule::DenyAll, MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply(); 
            let weapon_nft = ResourceBuilder::new_non_fungible()
                .metadata("type", "Substradix weapon NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .restrict_withdraw(AccessRule::AllowAll, MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
            let armor_nft = ResourceBuilder::new_non_fungible()
                .metadata("type", "Substradix weapon NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .restrict_withdraw(AccessRule::AllowAll, MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
            let accessory_nft = ResourceBuilder::new_non_fungible()
                .metadata("type", "Substradix weapon NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .restrict_withdraw(AccessRule::AllowAll, MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
                // Receipt NFT to store data of items listed for sale.
            let receipt_nft = ResourceBuilder::new_non_fungible()
                .metadata("type", "Substradix weapon NFT")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .restrict_withdraw(AccessRule::AllowAll, MUTABLE(developer_rule.clone()))
                .updateable_non_fungible_data(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
            // Gold for ingame currency
            let token_gold = ResourceBuilder::new_fungible()
                .metadata("name", "Gold Coin")
                .mintable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .burnable(system_rule.clone(), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
            // Materials for crafting
            let token_greavite = ResourceBuilder::new_fungible()
                .metadata("name", "Greavite Ore")
                .mintable(rule!(require(system_badge.resource_address())), MUTABLE(developer_rule.clone()))
                .burnable(rule!(require(system_badge.resource_address())), MUTABLE(developer_rule.clone()))
                .no_initial_supply();
            let token_wood = ResourceBuilder::new_fungible()
                .metadata("name", "Ageis Wood")
                .mintable(rule!(require(system_badge.resource_address())), MUTABLE(developer_rule.clone()))
                .burnable(rule!(require(system_badge.resource_address())), MUTABLE(developer_rule.clone()))
                .no_initial_supply();

            // Game Data can be hardcoded at instantiation, but I had it be set through transaction manifest for future use.
            // Game Data can always be updated through the Transaction Manifest, example file is setup.rtm.
            let game_data = structs::GameData {
                game_version:  dec!(1),
                game_price: game_price,
                character_number: 0,
                char_hp: HashMap::new(),
                char_atk: HashMap::new(),
                char_mag: HashMap::new(),
                char_def: HashMap::new(),
                char_spd: HashMap::new(),
                stage_data: HashMap::new(),
                exp_data: Vec::new(),
                weapon_data: HashMap::new(),
                armor_data: HashMap::new(),
                accessory_data: HashMap::new(),
            };

            let instantiate = Self {
                system_vault: Vault::with_bucket(system_badge),
                // Vault holds all Developer badeges except one given to instantiator.
                developer_vault: Vault::with_bucket(developer_badge.take(9999)),
                collected_xrd: Vault::new(RADIX_TOKEN),
                gold_vault: Vault::new(token_gold),
                character_nft,
                weapon_nft,
                armor_nft,
                accessory_nft,
                receipt_nft,
                token_greavite,
                token_wood,
                token_gold,
                marketplace_weapon_vault: Vault::new(weapon_nft),
                marketplace_armor_vault: Vault::new(armor_nft),
                marketplace_accessory_vault: Vault::new(accessory_nft),
                marketplace_listings: LazyMap::new(),
                game_data,
            }
            .instantiate();

            // Sets access for various methods
            let access_rules = AccessRules::new()
                .method("withdraw_xrd", rule!(require(developer_badge.resource_address())))
                .method("upload_stage_data", rule!(require(developer_badge.resource_address())))
                .method("upload_char_data", rule!(require(developer_badge.resource_address())))
                .method("change_price", rule!(require(developer_badge.resource_address())))
                .method("upload_levelup_data", rule!(require(developer_badge.resource_address())))
                .method("upload_weapon_data", rule!(require(developer_badge.resource_address())))
                .method("upload_armor_data", rule!(require(developer_badge.resource_address())))
                .method("upload_accessory_data", rule!(require(developer_badge.resource_address())))
                .method("create_character", rule!(allow_all))
                .method("fuse_items", rule!(allow_all))
                .method("create_weapon", rule!(allow_all))
                .method("create_armor", rule!(allow_all))
                .method("create_accessory", rule!(allow_all))
                .method("stage", rule!(allow_all))
                .method("combat", rule!(allow_all))
                .method("list_single_gear", rule!(allow_all))
                .method("buy_single_gear", rule!(allow_all))
                .method("redeem_receipt", rule!(allow_all))
                .method("change_listing_price", rule!(allow_all))
                .method("remove_listing", rule!(allow_all))
                .method("levelup", rule!(allow_all));
            
            (instantiate.add_access_check(access_rules).globalize(), developer_badge)
        }

        // Dev only, collects all XRD from sold Personal Tokens
        pub fn withdraw_xrd(&mut self) -> Bucket {
            self.collected_xrd.take_all()
        }
        // Changes price of Substradix
        pub fn change_price(&mut self, new_price: Decimal) {
            self.game_data.game_price = new_price;
        }
        // Upload data for the game
        pub fn upload_levelup_data(&mut self, data: Vec<u128>) {
            self.game_data.exp_data = data;
        }
        pub fn upload_weapon_data(&mut self, class: u64, id: Decimal, data: structs::WeaponData) {
            self.game_data.weapon_data.insert((class, id), data);
        }
        pub fn upload_armor_data(&mut self, armor: structs::ArmorNames, id: Decimal, data: structs::ArmorData) {
            self.game_data.armor_data.insert((armor, id), data);
        }

        pub fn upload_accessory_data(&mut self, accessory: structs::AccessoryNames,  id: Decimal, data: structs::AccessoryData) {
            self.game_data.accessory_data.insert((accessory, id), data);
        }
        pub fn upload_stage_data(&mut self, stage_number: u64, enemy1_data: structs::EnemyFullInfo, enemy2_data: structs::EnemyFullInfo, enemy3_data: structs::EnemyFullInfo,) {		                       			
            self.game_data.stage_data.insert(stage_number, vec![enemy1_data, enemy2_data, enemy3_data]);
        }
        // Upload Vectors to use for character stats
        pub fn upload_char_data(&mut self, class: u64, stat: structs::Stats, data: Vec<u64>) {
            match stat {
                structs::Stats::Health => self.game_data.char_hp.insert(class, data),
                structs::Stats::Attack => self.game_data.char_atk.insert(class, data),
                structs::Stats::Magic => self.game_data.char_mag.insert(class, data),
                structs::Stats::Defense => self.game_data.char_def.insert(class, data),
                structs::Stats::Speed => self.game_data.char_spd.insert(class, data),
            };
        }
        // Creates character.
        pub fn create_character(&mut self, mut payment: Bucket, class: u64, name: String) -> (Bucket, Bucket) {
            // Design: I used u64 for Characters instead of an Enum to support the creation of new characters without Blueprint updates.
            let hp = self.game_data.char_hp.get(&class).unwrap();
            let atk = self.game_data.char_atk.get(&class).unwrap();
            let mag = self.game_data.char_mag.get(&class).unwrap();
            let def = self.game_data.char_def.get(&class).unwrap();
            let spd = self.game_data.char_spd.get(&class).unwrap();
            let character_data = structs::Character { 
                name: name, 
                class: class, 
                level: 1, 
                exp: 0, 
                stage: dec!(1),
                health: hp[0].into(), 
                attack: atk[0].into(), 
                magic: mag[0].into(), 
                defense: def[0].into(), 
                speed: spd[0].into(), 
                version: self.game_data.game_version, 
            };
            ComponentAuthZone::push(self.system_vault.create_proof());

            let new_character = borrow_resource_manager!(self.character_nft)
                .mint_non_fungible(&NonFungibleId::from_u64(self.game_data.character_number), character_data);
            
            ComponentAuthZone::pop();

            self.game_data.character_number += 1;
            self.collected_xrd.put(payment.take(self.game_data.game_price));
            return (new_character, payment,)
        }
        // Takes two of the same type + level Weapon/Armor/Accessory NFT, burns them, and makes a new one based off of the first. 
        // Takes stats from first NFT. Stats increase by 20% per upgrade
        pub fn fuse_items(&mut self, item_bucket: Bucket) -> Bucket {
            assert!(item_bucket.amount() == dec!("2"));
            assert!(item_bucket.resource_address() == self.weapon_nft || 
            item_bucket.resource_address() == self.armor_nft ||
            item_bucket.resource_address() == self.accessory_nft);
            if item_bucket.resource_address() == self.weapon_nft {
                let mut item: structs::Weapon = item_bucket.non_fungibles()[0].data();
                let item2: structs::Weapon = item_bucket.non_fungibles()[1].data();
                assert!(item.item_info.level == item2.item_info.level);
                assert!(item.item_info.id == item2.item_info.id);
                item.item_info.level += 1;
                item.physical_base *= dec!("1.2");
                item.physical_scaling *= dec!("1.2");
                item.spell_base *= dec!("1.2");
                item.spell_scaling *= dec!("1.2"); 
                let new_info = structs::Item {
                    id: item.item_info.id,
                    version: self.game_data.game_version,
                    weight: item.item_info.weight,
                    level: item.item_info.level, 
                };         
                let new = structs::Weapon { class: item.class, physical_base: item.physical_base, item_info: new_info, 
                    ability: item.ability, ability_odds: item.ability_odds,
                    physical_scaling: item.physical_scaling, spell_base: item.spell_base, spell_scaling: item.spell_scaling, range: item.range, 
                };

                ComponentAuthZone::push(self.system_vault.create_proof());

                item_bucket.burn();
                let new_bucket = borrow_resource_manager!(self.weapon_nft)
                        .mint_non_fungible(&NonFungibleId::random(), new);
                        
                ComponentAuthZone::pop();
                return new_bucket
            }
            else if item_bucket.resource_address() == self.armor_nft {
                let mut item: structs::Armor = item_bucket.non_fungibles()[0].data();
                let item2: structs::Armor = item_bucket.non_fungibles()[1].data();   
                assert!(item.item_info.level == item2.item_info.level);           
                assert!(item.item_info.id == item2.item_info.id);
                assert!(item.part == item2.part);
                item.item_info.level += 1; 
                item.health *= dec!("1.25");
                item.defense *= dec!("1.25");
                let new_info = structs::Item {
                    id: item.item_info.id,
                    version: self.game_data.game_version,
                    weight: item.item_info.weight,
                    level: item.item_info.level, 
                };   
                let new = structs::Armor { part: item.part, health: item.health, defense: item.defense, item_info: new_info
                };

                ComponentAuthZone::push(self.system_vault.create_proof());

                item_bucket.burn();
                let new_bucket = borrow_resource_manager!(self.armor_nft)
                        .mint_non_fungible(&NonFungibleId::random(), new);

                ComponentAuthZone::pop();
                
                return new_bucket
            }
            else {
                let mut item: structs::Accessory = item_bucket.non_fungibles()[0].data();
                let item2: structs::Accessory = item_bucket.non_fungibles()[1].data();  
                assert!(item.item_info.level == item2.item_info.level);         
                assert!(item.item_info.id == item2.item_info.id);
                assert!(item.part == item2.part);
                item.item_info.level += 1;
                item.attack *= dec!("1.2");
                item.magic *= dec!("1.2");
                item.speed *= dec!("1.2");
                let new_info = structs::Item {
                    id: item.item_info.id,
                    version: self.game_data.game_version,
                    weight: item.item_info.weight,
                    level: item.item_info.level, 
                };   
                let new = structs::Accessory { 
                    part: item.part, 
                    attack: item.attack, 
                    magic: item.magic, 
                    speed: item.speed,
                    ability: item.ability, 
                    ability_odds: item.ability_odds, 
                    item_info: new_info
                };

                ComponentAuthZone::push(self.system_vault.create_proof());

                item_bucket.burn();
                let new_bucket = borrow_resource_manager!(self.accessory_nft)
                        .mint_non_fungible(&NonFungibleId::random(), new);

                ComponentAuthZone::pop();

                return new_bucket
            }              
        }
        // Creates weapons
        pub fn create_weapon(&mut self, mut gold: Bucket, mut resource1: Bucket, mut resource2: Bucket, class: u64, id: Decimal) -> (Bucket,Bucket,Bucket,Bucket) {
            let weapon_info = self.game_data.weapon_data.get(&(class,id)).unwrap();
            // Assertations so no cheating
            assert!(gold.resource_address() == self.token_gold);
            assert!(resource1.resource_address() == weapon_info.crafting_data.resource_1);
            assert!(resource2.resource_address() == weapon_info.crafting_data.resource_2);
            let gold_bucket: Bucket = gold.take(weapon_info.crafting_data.gold_cost);
            let resource1_bucket: Bucket = resource1.take(weapon_info.crafting_data.resource_1_cost);
            let resource2_bucket: Bucket = resource2.take(weapon_info.crafting_data.resource_2_cost);
            let seed = rng::seed(weapon_info.min_rng_roll, weapon_info.max_rng_roll);
            let ability = if seed == weapon_info.success_number || seed >= weapon_info.success_number {
                weapon_info.ability_if_success
            }
            else {
                dec!(0)
            };
            let info = structs::Item {
                id: id,
                version: self.game_data.game_version,
                weight: weapon_info.crafting_data.weight * rng::seed_decimal(75,125,dec!(100)),
                level: 1, 
            };  
            let weapon_data = structs::Weapon {  
                class: class,
                item_info: info,
                physical_base: weapon_info.physical_base * rng::seed_decimal(75,125,dec!(100)),
                physical_scaling: weapon_info.physical_scaling * rng::seed_decimal(75,125,dec!(100)),
                spell_base: weapon_info.magic_base * rng::seed_decimal(75,125,dec!(100)),
                spell_scaling: weapon_info.magic_scaling * rng::seed_decimal(75,125,dec!(100)),
                ability: ability,
                ability_odds: weapon_info.ability_active_odds * rng::seed_decimal(75,125,dec!(100)),
                range: weapon_info.range,
            };
            ComponentAuthZone::push(self.system_vault.create_proof());

            let new_weapon = borrow_resource_manager!(self.weapon_nft)
                .mint_non_fungible(&NonFungibleId::random(), weapon_data);
            gold_bucket.burn();
            resource1_bucket.burn();
            resource2_bucket.burn();

            ComponentAuthZone::pop();

            (new_weapon,gold,resource1,resource2)
        }
        pub fn create_armor(&mut self, mut gold: Bucket, mut resource1: Bucket, mut resource2: Bucket, armor: structs::ArmorNames, id: Decimal) -> (Bucket,Bucket,Bucket,Bucket) {
            let armor_info = self.game_data.armor_data.get(&(armor,id)).unwrap();
            let gold_bucket: Bucket = gold.take(armor_info.crafting_data.gold_cost);
            let resource1_bucket: Bucket = resource1.take(armor_info.crafting_data.resource_1_cost);
            let resource2_bucket: Bucket = resource2.take(armor_info.crafting_data.resource_2_cost);
            assert!(gold.resource_address() == self.token_gold);
            assert!(resource1.resource_address() == armor_info.crafting_data.resource_1);
            assert!(resource2.resource_address() == armor_info.crafting_data.resource_2);
            let chance = rng::seed(1, 2);
            let health = if chance == 1 { armor_info.health_bonus }
            else { armor_info.health };
            let defense = if chance == 2 { armor_info.defense_bonus }
            else { armor_info.defense };
            let info = structs::Item {
                id: id,
                version: self.game_data.game_version,
                weight: armor_info.crafting_data.weight * rng::seed_decimal(75,125,dec!(100)),
                level: 1, 
            };  
            let weapon_data = structs::Armor {  
                item_info: info,
                part: armor,
                health: health * rng::seed_decimal(75,125,dec!(100)),
                defense: defense * rng::seed_decimal(75,125,dec!(100)),
            };
            ComponentAuthZone::push(self.system_vault.create_proof());

            let new_weapon = borrow_resource_manager!(self.weapon_nft)
                .mint_non_fungible(&NonFungibleId::random(), weapon_data);
            gold_bucket.burn();
            resource1_bucket.burn();
            resource2_bucket.burn();

            ComponentAuthZone::pop();

            (new_weapon,gold,resource1,resource2)
        }
        pub fn create_accessory(&mut self, mut gold: Bucket, mut resource1: Bucket, mut resource2: Bucket, accessory: structs::AccessoryNames, id: Decimal) -> (Bucket,Bucket,Bucket,Bucket) {
            let accessory_info = self.game_data.accessory_data.get(&(accessory,id)).unwrap();
            let gold_bucket: Bucket = gold.take(accessory_info.crafting_data.gold_cost);
            let resource1_bucket: Bucket = resource1.take(accessory_info.crafting_data.resource_1_cost);
            let resource2_bucket: Bucket = resource2.take(accessory_info.crafting_data.resource_2_cost);
            assert!(gold.resource_address() == self.token_gold);
            assert!(resource1.resource_address() == accessory_info.crafting_data.resource_1);
            assert!(resource2.resource_address() == accessory_info.crafting_data.resource_2);
            let seed = rng::seed(accessory_info.min_rng_roll, accessory_info.max_rng_roll);
            let ability = if seed == accessory_info.success_number || seed >= accessory_info.success_number {
                accessory_info.ability_if_success
            }
            else {
                dec!(0)
            };
            let chance = rng::seed(1, 3);
            let attack = if chance == 1 { accessory_info.attack_bonus }
            else { accessory_info.attack };
            let magic = if chance == 2 { accessory_info.magic_bonus }
            else { accessory_info.magic };
            let speed = if chance == 3 { accessory_info.speed_bonus }
            else { accessory_info.speed };
            let info = structs::Item {
                id: id,
                version: self.game_data.game_version,
                weight: accessory_info.crafting_data.weight * rng::seed_decimal(75,125,dec!(100)),
                level: 1, 
            };  
            let weapon_data = structs::Accessory {  
                item_info: info,
                part: accessory,
                attack: attack * rng::seed_decimal(75,125,dec!(100)),
                magic: magic * rng::seed_decimal(75,125,dec!(100)),
                speed: speed * rng::seed_decimal(75,125,dec!(100)),
                ability: ability,
                ability_odds: accessory_info.ability_active_odds * rng::seed_decimal(75,125,dec!(100)),       
            };
            ComponentAuthZone::push(self.system_vault.create_proof());

            let new_weapon = borrow_resource_manager!(self.weapon_nft)
                .mint_non_fungible(&NonFungibleId::random(), weapon_data);
            gold_bucket.burn();
            resource1_bucket.burn();
            resource2_bucket.burn();

            ComponentAuthZone::pop();

            (new_weapon,gold,resource1,resource2)
        }
        // List gear on the marketplace. Prices are set in gold
        pub fn list_single_gear(&mut self, mut listing: Bucket, price: Decimal, category: structs::Categories) -> Bucket {
            let id = match category {
                structs::Categories::Weapon => listing.non_fungible::<structs::Weapon>().id(),
                structs::Categories::Armor => listing.non_fungible::<structs::Armor>().id(),
                structs::Categories::Accessory => listing.non_fungible::<structs::Accessory>().id(),
            };
            let new_id = NonFungibleId::random();
            let gear = listing.take(1);
            let receipt_data = structs::Receipt {
                price: price,
                item_id: id,
                category: category,
                id: new_id,
            };
            let new_receipt = borrow_resource_manager!(self.receipt_nft)
                .mint_non_fungible(&receipt_data.id, receipt_data.clone());
            self.marketplace_listings.insert((category, receipt_data.item_id.clone()), (receipt_data.clone(), false));
            match category {
                structs::Categories::Weapon => self.marketplace_weapon_vault.put(gear),
                structs::Categories::Armor => self.marketplace_armor_vault.put(gear),
                structs::Categories::Accessory => self.marketplace_accessory_vault.put(gear),
            };
            new_receipt
        }
        // Buy gear from the marketplace. Prices are set in gold
        pub fn buy_single_gear(&mut self, mut gold: Bucket, category: structs::Categories, id: NonFungibleId,) -> (Bucket, Bucket) {
            let nft = match category {
                structs::Categories::Armor => self.marketplace_armor_vault.take_non_fungible(&id),
                structs::Categories::Accessory => self.marketplace_accessory_vault.take_non_fungible(&id),
                structs::Categories::Weapon => self.marketplace_weapon_vault.take_non_fungible(&id),
            };
            let mut lazymap_data = self.marketplace_listings.get(&(category, id)).unwrap();
            assert!(lazymap_data.1 == false);
            lazymap_data.1 = true;
            let payment = gold.take(lazymap_data.0.price * dec!(".95"));
            let burn_bucket: Bucket = gold.take(lazymap_data.0.price * dec!(".05"));
            self.gold_vault.put(payment);
            self.marketplace_listings.insert((category, lazymap_data.0.id.clone()), lazymap_data);
            self.system_vault.authorize(|| 
                burn_bucket.burn());
            (gold,nft)
        }
        // Get your gold from a sold item
        pub fn redeem_receipt(&mut self, receipt: Bucket) -> Bucket {
            let receipt_data: structs::Receipt = receipt.non_fungible().data();
            let lazymap_data = self.marketplace_listings.get(&(receipt_data.category, receipt_data.item_id.clone())).unwrap();
            // No cheating!
            assert!(receipt.resource_address() == self.receipt_nft);
            assert!(lazymap_data.0 == receipt_data);
            assert!(lazymap_data.1 == true);
            // Can't remove a key pair from a LazyMap :( not exploitable but annoying af
            // self.marketplace_listings.remove(receipt_data.category, receipt_data.item_id);
            let gold = self.gold_vault.take(receipt_data.price * dec!(".95"));
            self.system_vault.authorize(|| 
                receipt.burn());
            gold
        }
        // Get your item back from listing
        pub fn remove_listing(&mut self, receipt: Bucket) -> Bucket {
            let receipt_data: structs::Receipt = receipt.non_fungible().data();
            let lazymap_data = self.marketplace_listings.get(&(receipt_data.category, receipt_data.item_id.clone())).unwrap();
            // No cheating!
            assert!(receipt.resource_address() == self.receipt_nft);
            assert!(lazymap_data.0 == receipt_data);
            assert!(lazymap_data.1 == false);
            // Can't remove a key pair from a LazyMap :( not exploitable but annoying af
            //self.marketplace_listings.remove(receipt_data.category, receipt_data.item_id);
            let nft = match lazymap_data.0.category {
                structs::Categories::Armor => self.marketplace_armor_vault.take_non_fungible(&lazymap_data.0.item_id),
                structs::Categories::Accessory => self.marketplace_accessory_vault.take_non_fungible(&lazymap_data.0.item_id),
                structs::Categories::Weapon => self.marketplace_weapon_vault.take_non_fungible(&lazymap_data.0.item_id),
            };
            self.system_vault.authorize(|| 
                receipt.burn());
            nft
        }
        pub fn change_listing_price(&mut self, receipt: Proof, new_price: Decimal) -> Decimal {
            let mut receipt_data: structs::Receipt = receipt.non_fungible().data();
            let lazymap_data = self.marketplace_listings.get(&(receipt_data.category, receipt_data.item_id.clone())).unwrap();
            // No cheating!
            assert!(receipt.resource_address() == self.receipt_nft);
            assert!(lazymap_data.0 == receipt_data);
            assert!(lazymap_data.1 == false);
            receipt_data.price = new_price;
            self.marketplace_listings.insert((receipt_data.category, receipt_data.item_id.clone()), (receipt_data.clone(), false));
            receipt_data.price
        }
        // Place character,weapon,armor, and accessory data + stage # to fight. 
        // Method calculates whether you win and grants rewards based on win or loss
        // Note: The Transaction Manifest currently does not support the placing a Proof inside of an Enum such as Option<Proof>.
        // While this code is sound within Scrypto, it cannot be testing with actual Proofs at the moment. However, it can be run using "None" as the Option<Proof>.
        pub fn stage(&mut self, 
            nft_proof: Proof, 
            weapon: Option<Proof>,
            helmet: Option<Proof>, 
            chest: Option<Proof>, 
            pants: Option<Proof>, 
            gloves: Option<Proof>, 
            belt: Option<Proof>, 
            shoes: Option<Proof>, 
            stage: u64,
            ) -> (Bucket, Bucket, Bucket) {
            // Data from Proofs
            let mut nft_data: structs::Character = nft_proof.non_fungible().data();
            // Sets gear data. Allows you to fight without any gear. Makes sure you're not using homebrew NFTs
            let weapon_data = match weapon {
                Some(weapon) => { 
                    assert!(weapon.resource_address() == self.weapon_nft);
                    weapon.non_fungible().data() },
                None => { let data = structs::Weapon::null_weapon(); data }
            };
            let helmet_data = match helmet {
                Some(helmet) => {
                    assert!(helmet.resource_address() == self.armor_nft);
                    helmet.non_fungible().data()},
                None => { let data = structs::Armor::null_armor(structs::ArmorNames::Helmet); data }
            };
            let chest_data = match chest {
                Some(chest) =>{ 
                    assert!(chest.resource_address() == self.armor_nft);
                    chest.non_fungible().data()},
                None => { let data = structs::Armor::null_armor(structs::ArmorNames::Chest); data }
            };
            let pants_data = match pants {
                Some(pants) => {
                    assert!(pants.resource_address() == self.armor_nft);
                    pants.non_fungible().data()},
                None => { let data = structs::Armor::null_armor(structs::ArmorNames::Pants); data }
            };
            let gloves_data = match gloves {
                Some(gloves) => {
                    assert!(gloves.resource_address() == self.accessory_nft);
                    gloves.non_fungible().data()},
                None => { let data = structs::Accessory::null_accessory(structs::AccessoryNames::Gloves); data }
            };
            let belt_data = match belt {
                Some(belt) => {
                    assert!(belt.resource_address() == self.accessory_nft);
                    belt.non_fungible().data()},
                None => { let data = structs::Accessory::null_accessory(structs::AccessoryNames::Belt); data }
            };
            let shoes_data = match shoes {
                Some(shoes) => {
                    assert!(shoes.resource_address() == self.accessory_nft);
                    shoes.non_fungible().data()},
                None => { let data = structs::Accessory::null_accessory(structs::AccessoryNames::Shoes); data }
            };
            // Getting data of selected stage:
            let data = self.game_data.stage_data.get(&stage).unwrap().clone();
            let enemy_1_data = &data[0];
            let enemy_2_data = &data[1];
            let enemy_3_data = &data[2];
            // Assertions so no using different gear on wrong parts, character NFT is right, and stage progession is right
            assert!(nft_proof.resource_address() == self.character_nft,);
            assert!(nft_data.stage >= stage.into() || nft_data.stage == stage.into());
            assert!(helmet_data.part == structs::ArmorNames::Helmet);
            assert!(chest_data.part == structs::ArmorNames::Chest);
            assert!(pants_data.part == structs::ArmorNames::Pants);
            assert!(gloves_data.part == structs::AccessoryNames::Gloves);
            assert!(belt_data.part == structs::AccessoryNames::Belt);
            assert!(shoes_data.part == structs::AccessoryNames::Shoes);
            // Speed = sum of character + gear speed/Speed penality (Tier1 gear gives 1% penalty per item for 7% total penalty)
            let speed = (nft_data.speed + gloves_data.speed + belt_data.speed + shoes_data.speed) * (dec!(1) - weapon_data.item_info.weight -
                helmet_data.item_info.weight - chest_data.item_info.weight - pants_data.item_info.weight - 
                gloves_data.item_info.weight - belt_data.item_info.weight - shoes_data.item_info.weight);
            // Defense = structs::Character defense * gear buff
            let defense = {
                nft_data.defense * helmet_data.defense * chest_data.defense * pants_data.defense
            };
            // Attack = structs::Character attack * gear buff
            let attack = {
                nft_data.attack * gloves_data.attack * belt_data.attack * shoes_data.attack
            };
            // Magic = structs::Character magic * gear buff
            let magic = {
                nft_data.magic * gloves_data.magic * belt_data.magic * shoes_data.magic
            };
            // Health, like Speed, is simply added together. However, there are no penalties for Health like Speed
            let health = nft_data.health + helmet_data.health + chest_data.health + pants_data.health;
            let damage: Decimal = 
                (weapon_data.physical_base + (weapon_data.physical_scaling * attack)) +
                (weapon_data.spell_base + (weapon_data.spell_scaling * magic));
            let weapon_ability: Option<(Decimal,Decimal,Decimal)> = if weapon_data.ability == dec!(0) { None }
            else { Some((weapon_data.ability, weapon_data.ability_odds, damage))
            };
            let gloves_ability: Option<(Decimal,Decimal,Decimal)> = if gloves_data.ability == dec!(0) { None }
            else { Some((gloves_data.ability, gloves_data.ability_odds, damage))
            };
            let belt_ability: Option<(Decimal,Decimal,Decimal)> = if belt_data.ability == dec!(0) { None }
            else { Some((belt_data.ability, belt_data.ability_odds, damage))
            };
            let shoes_ability: Option<(Decimal,Decimal,Decimal)> = if shoes_data.ability == dec!(0) { None }
            else { Some((shoes_data.ability, shoes_data.ability_odds, damage))
            };
            let mut player_info = structs::CombatInfo {
                health: health,
                damage: damage,
                defense: defense,
                speed: speed,
                ability_weapon: weapon_ability,
                ability_gloves: gloves_ability,
                ability_belt: belt_ability,
                ability_shoes: shoes_ability,
            };
            // To modify combat, simply change numbers for Enemy Data, EXP rewards, and Stage Number.
            let fight = combat::combat(player_info, enemy_1_data.combat_info);
            player_info.health = fight;
            let fight2 = combat::combat(player_info, enemy_2_data.combat_info);
            player_info.health = fight2;
            let fight3 = combat::combat(player_info, enemy_3_data.combat_info);
            // To modify stage rewards, simply change numbers + minted rewards below
            // Numbers which drop can be randomized as well, simply add self.seed_
            let rewards = if fight == dec!(0) || fight <= dec!(0) {
                let exp = enemy_1_data.exp_on_loss;
                let gold = enemy_1_data.gold_on_loss;
                let greavite = enemy_1_data.greavite_on_loss;
                let wood = enemy_1_data.wood_on_loss;
                (exp,gold,greavite,wood)
            }
            else if fight2 == dec!(0) || fight2 <= dec!(0) {
                let exp = enemy_1_data.exp_on_win + enemy_2_data.exp_on_loss;
                let gold = enemy_1_data.gold_on_win + enemy_2_data.gold_on_loss;
                let greavite = enemy_1_data.greavite_on_win + enemy_2_data.greavite_on_loss;
                let wood = enemy_1_data.wood_on_win + enemy_2_data.wood_on_loss;
                (exp,gold,greavite,wood)
            }
            else if fight3 == dec!(0) || fight3 <= dec!(0) {
                let exp = enemy_1_data.exp_on_win + enemy_2_data.exp_on_win + enemy_3_data.exp_on_loss;
                let gold = enemy_1_data.gold_on_win + enemy_2_data.gold_on_win + enemy_3_data.gold_on_loss;
                let greavite = enemy_1_data.greavite_on_win + enemy_2_data.greavite_on_win + enemy_3_data.greavite_on_loss;
                let wood = enemy_1_data.wood_on_win + enemy_2_data.wood_on_win + enemy_2_data.wood_on_loss;
                (exp,gold,greavite,wood)
            }
            else {
                let exp = enemy_1_data.exp_on_win + enemy_2_data.exp_on_win + enemy_3_data.exp_on_win;
                let gold = enemy_1_data.gold_on_win + enemy_2_data.gold_on_win + enemy_3_data.gold_on_win;
                let greavite = enemy_1_data.greavite_on_win + enemy_2_data.greavite_on_win + enemy_3_data.greavite_on_win;
                let wood = enemy_1_data.wood_on_win + enemy_2_data.wood_on_win + enemy_2_data.wood_on_win;
                (exp,gold,greavite,wood)
            };
            nft_data.exp += rewards.0;
                let reward1 = self.system_vault.authorize(||
                    borrow_resource_manager!(self.token_gold)
                        .mint(rewards.1));
                let reward2 = self.system_vault.authorize(||
                        borrow_resource_manager!(self.token_wood)
                        .mint(rewards.2));
                let reward3 = self.system_vault.authorize(||
                    borrow_resource_manager!(self.token_greavite)
                        .mint(rewards.3));
                nft_data = self.levelup(nft_data.clone());
                self.system_vault.authorize(|| nft_proof.non_fungible().update_data(nft_data));
                return (reward1, reward2, reward3)
        }
        //Levelup method
        pub fn levelup(&mut self, nft_data: structs::Character) -> structs::Character {
            let hp = self.game_data.char_hp.get(&nft_data.class).unwrap();
            let atk = self.game_data.char_atk.get(&nft_data.class).unwrap();
            let mag = self.game_data.char_mag.get(&nft_data.class).unwrap();
            let def = self.game_data.char_def.get(&nft_data.class).unwrap();
            let spd = self.game_data.char_spd.get(&nft_data.class).unwrap();
            let mut level: usize = nft_data.level.try_into().unwrap();
            if nft_data.exp >= self.game_data.exp_data[level - 1] && nft_data.exp < self.game_data.exp_data[level] { 
                return nft_data;
            };
            let mut new_data = nft_data.clone();
            // Loops levelups until you reach the level for your given EXP
            loop {
                if nft_data.exp >= self.game_data.exp_data[level] { 
                    new_data = structs::Character {
                        name: nft_data.name.clone(),
                        class: nft_data.class,
                        level: level as u64 + 1,
                        exp: nft_data.exp,
                        stage: nft_data.stage, 
                        health: hp[level].into(),
                        attack: atk[level].into(),
                        magic: mag[level].into(),
                        defense: def[level].into(),
                        speed: spd[level].into(),
                        version: self.game_data.game_version,
                    };
                    level += 1;
                    continue; 
                }
                else {
                    return new_data
                }
            }
        }
    }
}
