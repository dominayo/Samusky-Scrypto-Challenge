use scrypto::prelude::*;
use super::structs;
use super::rng;

// Calculates the combat between player and enemy. Returns player health
pub fn combat(mut player_stats: structs::CombatInfo, mut enemy_stats: structs::CombatInfo ) -> Decimal {
    let mut rounds: u16 = 0;
    let mut hits: u16 = 0;
    let mut enemy_hits: u16 = 0;
    // Loop which runs until one of the fighter's health is 0
    'outer: loop {
        rounds += 1;
        info!("Turn {}:", rounds);
        let damage_given = std::cmp::max(Decimal::one(), player_stats.damage - (enemy_stats.defense / dec!(2)));
        let damage_taken = std::cmp::max(Decimal::one(), enemy_stats.damage - (player_stats.defense / dec!(2)));
        let mut priority = player_stats.speed / enemy_stats.speed * rng::seed_decimal(75,125,dec!(100));
        // Check if a character has died.
        if player_stats.health <= dec!(0) {
            info!("PlayerDies");
            return player_stats.health
        };
        if enemy_stats.health <= dec!(0) {
            info!("EnemyDies");
            return player_stats.health
        };
        if priority >= dec!(1) {
            info!("Player attacks");
            // Loop which implements the amount of hits the player can do, depending on priority. 
            loop {
                // Applies random variance to the base damage, and rounds that damage nummber to the nearest integer
                enemy_stats.health -= Decimal::round(&(damage_given * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero); 
                hits += 1;
                info!("Hits: {}", hits);
                info!("Health: {} Enemy Health: {}", player_stats.health, enemy_stats.health);
                if player_stats.health <= dec!(0) {
                    info!("PlayerDies");
                    return player_stats.health
                };
                if enemy_stats.health <= dec!(0) {
                    info!("EnemyDies");
                    return player_stats.health
                };
                // Changes priority to limit the amount of attacks per turn
                priority *= dec!(".9");
                if priority >= dec!(1) {
                    continue;
                }
                // Once priority is below 1, the enemy gets a hit and the round is over.
                else {
                    info!("Enemy attacks");
                    player_stats.health -= Decimal::round(&(damage_taken * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                    enemy_hits = 1;
                    info!("Enemy hits: {}", enemy_hits);
                    info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                    if player_stats.health < dec!(0) || player_stats.health == dec!(0) {
                        info!("PlayerDies");
                        return player_stats.health
                    }
                    if enemy_stats.health < dec!(0) || enemy_stats.health == dec!(0) {
                        info!("EnemyDies");
                        return player_stats.health
                    }
                    continue 'outer;
                }
            }
        }
        else {
            info!("Enemy attacks");
            // Loop which implements the amount of hits the enemy can do
            loop {
                player_stats.health -= Decimal::round(&(damage_taken * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                enemy_hits += 1;
                info!("Enemy hits: {}", enemy_hits);
                info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                if player_stats.health <= dec!(0) {
                    info!("PlayerDies");
                    return player_stats.health
                }
                if enemy_stats.health <= dec!(0) {
                    info!("EnemyDies");
                    return player_stats.health
                }
                priority *= dec!("1.15");
                if priority < dec!(1) {
                continue;
                }
                else {
                    info!("Player attacks");
                    enemy_stats.health -= Decimal::round(&(damage_given * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                    hits = 1;
                    info!("Hits: {}", hits);
                    info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                    if player_stats.health <= dec!(0){
                        info!("PlayerDies");
                        return player_stats.health
                    }
                    if enemy_stats.health <= dec!(0) {
                        info!("EnemyDies");
                        return player_stats.health
                    }
                    continue 'outer;
                }
            }
        };
    }
}
