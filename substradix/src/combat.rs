use scrypto::prelude::*;
use super::structs;
use super::rng;

// Calculates the combat between user and enemy. Enemy Data + rewards set by caller. Returns health after combat
pub fn combat( mut player_stats: structs::CombatInfo, mut enemy_stats: structs::CombatInfo ) -> Decimal {
    let mut rounds: u128 = 0;
    let mut hits: u128 = 0;
    loop {
        rounds += 1;
        info!("Round {}:", rounds);
        let damage_given = std::cmp::max(Decimal::one(), player_stats.damage - (enemy_stats.defense / dec!(2)));
        let damage_taken = std::cmp::max(Decimal::one(), enemy_stats.damage - (player_stats.defense / dec!(2)));
        let mut priority = player_stats.speed / enemy_stats.speed * rng::seed_decimal(75,125,dec!(100));
        info!("Damage Given: {}", damage_given);
        info!("Damage taken: {}", damage_taken);
        info!("Prio: {}", priority);
        if player_stats.health <= dec!(0) {
            info!("PlayerDies");
            return player_stats.health
            
        };
        if enemy_stats.health <= dec!(0) {
            info!("EnemyDies");
            return player_stats.health
        };
        if priority >= dec!(1) {
            loop {
                info!("Player attacks");
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
                priority *= dec!(".9");
                if priority >= dec!(1) {
                    continue;
                }
                else {
                    info!("Enemy attacks");
                    player_stats.health -= Decimal::round(&(damage_taken * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                    hits = 0;
                    info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                        if player_stats.health < dec!(0) || player_stats.health == dec!(0) {
                            info!("PlayerDies");
                            return player_stats.health
                        }
                        if enemy_stats.health < dec!(0) || enemy_stats.health == dec!(0) {
                            info!("EnemyDies");
                            return player_stats.health
                        }
                        break;
                }
            }
        };
        if priority < dec!(1) {
            info!("Enemy attacks");
            hits = 0;
            loop {
                player_stats.health -= Decimal::round(&(damage_taken * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                hits += 1;
                info!("Enemy hits: {}", hits);
                info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                if player_stats.health <= dec!(0) {
                    info!("PlayerDies");
                    return player_stats.health
                }
                if enemy_stats.health <= dec!(0) {
                    info!("EnemyDies");
                    return player_stats.health
                }
                priority *= dec!("1.1");
                if priority < dec!(1) {
                continue;
                }
                else {
                    info!("Player attacks");
                    enemy_stats.health -= Decimal::round(&(damage_given * rng::seed_decimal(75,125,dec!(100))), 0, RoundingMode::TowardsNearestAndHalfTowardsZero);
                    hits = 1;
                    info!("Hits: {}", hits);
                    info!("Health: {} Enemy Health: {}", player_stats.health,enemy_stats.health);
                    if player_stats.health < dec!(0) || player_stats.health == dec!(0) {
                        return player_stats.health
                    }
                    if enemy_stats.health < dec!(0) || enemy_stats.health == dec!(0) {
                        return player_stats.health
                    }
                    break;
                }
            }
        };
    }
}