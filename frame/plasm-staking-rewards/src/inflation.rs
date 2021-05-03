//! This is explains plasm inflation models.
//! The staking has 2 kinds.
//!
//! 1. Validator Staking
//! 2. Dapps(Operator) Staking
//!
//! About each staking, this module computes issuing new tokens.

use super::*;
//use traits::ComputeTotalPayout;
use num_traits::sign::Unsigned;
use sp_arithmetic::traits::BaseArithmetic;

pub fn compute_total_rewards<T: crate::Config>(
        total_tokens: BalanceOf<T>,
        era_duration: u64,
        number_of_validator: u32,
        _dapps_staking: u32,
    ) -> (BalanceOf<T>, BalanceOf<T>)
    where
        T: crate::Config,
{
    const TARGETS_NUMBER: u128 = 100;
    const MILLISECONDS_PER_YEAR: u128 = 1000 * 3600 * 24 * 36525 / 100;
    // I_0 = 2.5%.
    const I_0_DENOMINATOR: u128 = 25;
    const I_0_NUMERATOR: u128 = 1000;
    let number_of_validator_clone: u128 = number_of_validator.clone().into();
    let era_duration_clone: u128 = era_duration.clone().into();
    let number_of_validator: u128 = number_of_validator.into();
    let portion = if TARGETS_NUMBER < number_of_validator_clone {
        // TotalForSecurityRewards
        // = TotalAmountOfIssue * I_0% * (EraDuration / 1year)

        // denominator: I_0_DENOMINATOR * EraDuration
        // numerator: 1year * I_0_NUMERATOR
        Perbill::from_rational_approximation(
            I_0_DENOMINATOR * era_duration_clone,
               MILLISECONDS_PER_YEAR * I_0_NUMERATOR,
        )
    } else {
        // TotalForSecurityRewards
        // = TotalAmountOfIssue * I_0% * (NumberOfValidators/TargetsNumber) * (EraDuration/1year)

        // denominator: I_0_DENOMINATOR * NumberOfValidators * EraDuration
        // numerator: 1year * I_0_NUMERATOR * TargetsNumber
        Perbill::from_rational_approximation(
            I_0_DENOMINATOR * number_of_validator * era_duration_clone,
            MILLISECONDS_PER_YEAR * I_0_NUMERATOR * TARGETS_NUMBER,
        )
    };
    let payout = portion * total_tokens;
    (payout, total_tokens - payout) // (validator reward, dapps staking reward)
}

#[cfg(test)]
mod test {
    use super::*;

    fn compute_total_rewards_test<N>(
        total_tokens: N,
        era_duration: u64,
        number_of_validators: u32,
        _dapps_staking: u32,
    ) -> (N, N)
    where
        N: BaseArithmetic + Unsigned + Clone + From<u32>,
    {
        compute_total_rewards(total_tokens, era_duration, number_of_validators, _dapps_staking)
    }

    #[test]
    fn test_compute_total_reward() {
        const YEAR: u64 = 365 * 24 * 60 * 60 * 1000;
        // check maximum inflation.
        // not 10_000 due to rounding error.
        assert_eq!(compute_total_rewards_test(100_000_000, YEAR, 0, 0).0, 19_986_311);
/*
        const DAY: u64 = 24 * 60 * 60 * 1000;
        assert_eq!(compute_total_rewards::<T>(BalanceOf::<T>::from(100_000_000), DAY, 0, 0).0, 54_757);

        const SIX_HOURS: u64 = 6 * 60 * 60 * 1000;
        assert_eq!(compute_total_rewards::<T>(BalanceOf::<T>::from(100_000_000), SIX_HOURS, 0, 0).0, 13_689);

        const HOUR: u64 = 60 * 60 * 1000;
        assert_eq!(compute_total_rewards::<T>(BalanceOf::<T>::from(100_000_000), HOUR, 0, 0).0, 2_281);
        */
    }

}
