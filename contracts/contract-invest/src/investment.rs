use crate::*;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Investment {
    // how much was invested
    pub amount: Balance,
    // starting value when investment was created
    pub baseline_index: f64,
    // when this investment was made - in UNIX seconds UTC
    pub invested_time: u64,
    // when this investment can be claimed - in UNIX seconds UTC
    pub maturity_time: u64,
}

impl Investment {
    /// whether or not this investment has reached maturity date and can be withdrawn
    pub fn is_mature(&self) -> bool {
        (env::block_timestamp() / 1000_000_000) >= self.maturity_time
    }

    /// calculates what this would return if it was mature and the data was recent enough
    /// just for displaying in UI.
    pub fn would_reward(&self, loc: &Location) -> Balance {
        if let measure = &loc.cur_index {
            // measurement after maturity, within window
            // calculate ratio, positive, if measurement below baseline
            // no code to divide Decimals, so we do this
            let ratio = self.baseline_index / measure.value;
            self.amount * ratio as Balance
        } else {
            0
        }
    }

    /// calculates the reward. if it is not mature, or there is insufficient data
    /// to provide a result, then it will return None
    pub fn reward(&self, loc: &Location, measurement_window: u64) -> Option<Balance> {
        if !self.is_mature() {
            return None;
        }
        // TODO: we need to store historical data... you cannot just wait it out
        if let measure = &loc.cur_index {
            match measure.time.checked_sub(self.maturity_time) {
                Some(val) if val <= measurement_window * 86400 => {
                    // measurement after maturity, within window
                    // calculate ratio, positive, if measurement below baseline
                    // no code to divide Decimals, so we do this
                    let ratio = self.baseline_index / measure.value;
                    let reward = self.amount * ratio as Balance;
                    Some(reward)
                }
                Some(_) => {
                    // measurement after maturity, after window, return 100%
                    Some(self.amount)
                }
                None => {
                    // measurement before maturity date
                    None
                }
            }
        } else {
            None
        }
    }
}
