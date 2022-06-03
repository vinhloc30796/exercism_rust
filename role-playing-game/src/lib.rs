// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        //! Revive the player if they are dead.
        //! Otherwise, don't revive the player.
        //! If level is 10 or above, the player has 100 mana.
        //! If level is less than 10, the player has no mana.
        match (self.health, self.level) {
            (0, 0..=9) => Some(Player {
                health: 100,
                mana: None,
                level: self.level,
            }),
            (0, 10..) => Some(Player {
                health: 100,
                mana: Some(100),
                level: self.level,
            }),
            (1.., _) => None,
            _ => None, // Not sure if this is correct
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        //! Checks if the player has enough mana to cast the spell.
        //! If they do, return the damage - defined as 2 times the mana cost.
        //! If they don't, return 0.
        //! If the player doesn't have mana, then decrease their health by mana_cost
        //! and return 0
        match self.mana {
            Some(mana) => {
                if mana >= mana_cost {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
            None => {
                // Damage should be at most health
                let damage = mana_cost.min(self.health);
                self.health -= damage;
                0
            }
        }
    }
}
