use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct GlobalPool {
    pub super_admin: Pubkey, // 32
    pub total_round: u64,    // 8
}

#[zero_copy]
#[derive(Default, PartialEq)]
#[repr(packed)]
pub struct GameData {
    pub play_time: i64,     // 8
    pub amount: u64,        // 8
    pub reward_amount: u64, // 8
    pub choice: u64,       // 8
    pub rand: u64,          // 8
}

#[account(zero_copy)]
pub struct PlayerPool {
    // 104
    pub player: Pubkey,       // 32
    pub round: u64,           // 8
    pub game_data: GameData,  // 40
    pub win_times: u64,       // 8
    pub reveived_reward: u64, // 8
}

impl Default for PlayerPool {
    #[inline]
    fn default() -> PlayerPool {
        PlayerPool {
            player: Pubkey::default(),
            round: 0,
            game_data: GameData {
                play_time: 0,
                amount: 0,
                reward_amount: 0,
                choice: 0,
                rand: 0,
            },
            win_times: 0,
            reveived_reward: 0,
        }
    }
}

impl PlayerPool {
    pub fn add_game_data(&mut self, now: i64, amount: u64, reward: u64, choice: u64, rand: u64) {
        self.game_data.play_time = now;
        self.game_data.amount = amount;
        self.game_data.reward_amount = reward;
        self.game_data.choice = choice;
        self.game_data.rand = rand;
        self.round += 1;
        if reward > 0 {
            self.win_times += 1;
            self.reveived_reward += reward;
        }
    }
}