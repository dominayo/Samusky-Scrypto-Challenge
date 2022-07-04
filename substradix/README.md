# Substradix - a Play-to-Earn Scrypto Autobattler

Substradix is a fully on-chain Play-to-Earn Game utilizing NFT and Token assets to play the game. Because all assets are tokenized, they can freely be traded, staked, 
and used to earn valuable tokens to progress within the game.

### Gameplay

The Basic Gameplay loop is as follows:

- Users create a character
- Users battle with that character, getting stronger from exp and getting crafting resources.
- Users craft better gear/weapons/accessories with resources
- Users combine crafted gear/weapons/accessories to futher stregthen themselves.
- Users battle tougher and tougher enemies, requiring better gear/accessories/weapons, etc.

Outside of the core Gameplay Loop (Implmentation in progress):
- Users sell gear for gold, which can be staked or exchanged for XRD.
- Users earn cool skins/titles
- Users fight in asynchronous PVP or Guild battles for rewards

### Getting started

The Demo of the game takes place through Resim, the Radix Engine Simulator. To assist in creating transactions, several sample Transaction Manifest Files have been 
created in the `manifest` folder. 

To begin: 

Reset resim to clear out gunk, and create a new account. Save the account address.
```
resim reset
resim new-account
account = [account_address]
```

Publish the package, and then instantiate the Substradix Component
```
resim publish .
package = [package_address]
resim call-function $package Substradix new 100
$component = [component_address]
```

Place variables into the Transaction Manifest file `setup.rtm`

Because game data is not hardcoded for reabability and development reasons, the game data must be set post-instantiation.

in `setup.rtm`, replace the account, component, and resource addresses with the necessary addresses

Afterwards, execute setup.rtm
```
resim run manifest/setup.rtm
```
Now all the data is set up!

Next, create a character! 

Either run
```
resim call-method $component create_character 100,030000000000000000000000000000000000000000000000000004 1 [character_name]
```

Or change the addresses in `create_character.rtm` and run
```
resim run manifest/create_character.rtm
````

By now, you should have the hang of this. Here's a list of methods to run next, and the corresponding Transaction Manifest File:

### Methods/Manifest Files to run
- stage | `manifest/combat.rtm`
 
|**NOTE**| Neither the Transaction Manifest nor the command line currently support the type `Option<Proof>`. |
|----|-----|

|To run the "stage" method with Proofs as gear data, a separate Transaction Manifest file must be run: `full_proof_combat.rtm`.
|----|

| This file will only work if every gear type is owned and inputed as a Proof: `Weapon`, `Helmet`, `Chest`,`Pants`,`Gloves`, `Belt`, and `Shoes`.
|----|

`
wen Proof in Enum support :'(`

- create_weapon | `manifest/forge_weapon.rtm`
- create_armor | `manifest/forge_armor.rtm`
- create_accessory | `manifest/forge_accessory.rtm`
- fuse_items | `manifest/fuse_items.rtm`
- list_single_gear | `manifest/list_gear.rtm`
- buy_single_gear | `manifest/buy_gear.rtm`
- redeem_receipt | `manifest/redeem_sale.rtm`
- remove_listing | `manifest/remove_listing.rtm`
- change_listing_price | `manifest/change_listing_price.rtm`
- create_character | `manifest/create_character.rtm`
- change_price | `manifest/change_game_price.rtm`
- withdraw_xrd | `manifest/withdraw_xrd.rtm`
- upload_levelup_data | `manifest/setup.rtm`
- upload_weapon_data | `manifest/setup.rtm`
- upload_armor_data | `manifest/setup.rtm`
- upload_accessory_data | `manifest/setup.rtm`
- upload_stage_data | `manifest/setup.rtm`
- upload_char_data | `manifest/setup.rtm`
