use crate::{
    types::{shared::Timeframe, volumetrics::Volumetric},
    utils::balance_strings::merge_bal_vecs,
};

pub fn aggregate_volumetrics(
    volumetrics: &Vec<Volumetric>,
    time_period: &Timeframe,
) -> Vec<Volumetric> {
    let mut initial = volumetrics[0].clone();
    initial.timestamp = Timeframe::round_timestamp(time_period, &initial.timestamp);

    let x = volumetrics[1..]
        .iter()
        .fold(vec![initial], |mut acc, volumetric| {
            let len = acc.len();
            let current = &mut acc[len - 1];

            let volume_period_ts = Timeframe::round_timestamp(time_period, &current.timestamp);

            let same_period = volume_period_ts == current.timestamp;

            if same_period {
                current.mint = merge_bal_vecs(&current.mint, &volumetric.mint);
                current.withdrawal =
                    merge_bal_vecs(&current.withdrawal, &volumetric.withdrawal);
                current.swaps_in = merge_bal_vecs(&current.swaps_in, &volumetric.swaps_in);
                current.swaps_out =
                    merge_bal_vecs(&current.swaps_out, &volumetric.swaps_out);
                current.transfer = &current.transfer + &volumetric.transfer;
                current.withdrawal = merge_bal_vecs(&current.withdrawal, &volumetric.withdrawal);
                current.mint = merge_bal_vecs(&current.mint, &volumetric.mint);
                // current.withdrawal = vec![
                //     add_bal_string_to_bal_string(&current.withdrawal[0], &volumetric.withdrawal[0]),
                //     add_bal_string_to_bal_string(&current.withdrawal[1], &volumetric.withdrawal[1]),
                // ];
                // current.mint = vec![
                //     add_bal_string_to_bal_string(&current.mint[0], &volumetric.mint[0]),
                //     add_bal_string_to_bal_string(&current.mint[1], &volumetric.mint[1]),
                // ];
            } else {
                let mut new_period_volume = volumetric.clone();
                new_period_volume.timestamp = volume_period_ts;
                acc.push(new_period_volume);
            }
            acc
        });
    x
}

pub fn create_period_volume(
    volumetrics: &Vec<Volumetric>,
    time_period: &Timeframe,
) -> Volumetric {
    let mut initial = volumetrics[0].clone();
    initial.timestamp = Timeframe::round_timestamp(time_period, &initial.timestamp);

    let x = volumetrics[1..]
        .iter()
        .fold(initial, |mut acc, volumetric| {
            acc.mint = merge_bal_vecs(&acc.mint, &volumetric.mint);
            acc.withdrawal = merge_bal_vecs(&acc.withdrawal, &volumetric.withdrawal);
            acc.swaps_in = merge_bal_vecs(&acc.swaps_in, &volumetric.swaps_in);
            acc.swaps_out = merge_bal_vecs(&acc.swaps_out, &volumetric.swaps_out);
            acc.transfer =&acc.transfer + &volumetric.transfer;
            acc.withdrawal = merge_bal_vecs(&acc.withdrawal, &volumetric.withdrawal);
            acc.mint = merge_bal_vecs(&acc.mint, &volumetric.mint);
            // acc.withdrawal = vec![
            //     add_bal_string_to_bal_string(&acc.withdrawal[0], &volumetric.withdrawal[0]),
            //     add_bal_string_to_bal_string(&acc.withdrawal[1], &volumetric.withdrawal[1]),
            // ];
            // acc.mint = vec![
            //     add_bal_string_to_bal_string(&acc.mint[0], &volumetric.mint[0]),
            //     add_bal_string_to_bal_string(&acc.mint[1], &volumetric.mint[1]),
            // ];

            acc
        });
    x
}
