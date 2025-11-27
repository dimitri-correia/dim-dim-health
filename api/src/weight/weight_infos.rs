use chrono::{Duration, Utc};
use entities::user_weight;
use sea_orm::prelude::Decimal;

use crate::schemas::user_weight_schemas::{UserWeightInfosResponse, UserWeightResponse};

pub fn user_weight_infos(
    all_user_weights: Vec<user_weight::Model>,
) -> Option<UserWeightInfosResponse> {
    if all_user_weights.is_empty() {
        return None;
    }

    // Sort weights by recorded_at in descending order (most recent first)
    let mut sorted_weights = all_user_weights.clone();
    sorted_weights.sort_by(|a, b| b.recorded_at.cmp(&a.recorded_at));

    let last_3_weights: Vec<UserWeightResponse> = sorted_weights
        .iter()
        .take(3)
        .map(|w| UserWeightResponse::from(w.clone()))
        .collect();

    let number_of_weight_entries = sorted_weights.len() as i64;
    let total_weight: Decimal = sorted_weights.iter().map(|w| w.weight_in_kg).sum();
    let average_weight = if number_of_weight_entries > 0 {
        total_weight / Decimal::from(number_of_weight_entries)
    } else {
        Decimal::ZERO
    };

    let seven_days_ago = (Utc::now() - Duration::days(7)).date_naive();
    let weights_last_7_days: Vec<_> = sorted_weights
        .iter()
        .filter(|w| w.recorded_at >= seven_days_ago)
        .collect();
    let number_of_weight_entries_last_7_days = weights_last_7_days.len() as i64;
    let total_weight_last_7_days: Decimal =
        weights_last_7_days.iter().map(|w| w.weight_in_kg).sum();
    let average_weight_last_7_days = if number_of_weight_entries_last_7_days > 0 {
        total_weight_last_7_days / Decimal::from(number_of_weight_entries_last_7_days)
    } else {
        Decimal::ZERO
    };

    let thirty_days_ago = (Utc::now() - Duration::days(30)).date_naive();
    let weights_last_30_days: Vec<_> = sorted_weights
        .iter()
        .filter(|w| w.recorded_at >= thirty_days_ago)
        .collect();
    let number_of_weight_entries_last_30_days = weights_last_30_days.len() as i64;
    let total_weight_last_30_days: Decimal =
        weights_last_30_days.iter().map(|w| w.weight_in_kg).sum();
    let average_weight_last_30_days = if number_of_weight_entries_last_30_days > 0 {
        total_weight_last_30_days / Decimal::from(number_of_weight_entries_last_30_days)
    } else {
        Decimal::ZERO
    };

    let max_weight_entry = sorted_weights
        .iter()
        .max_by(|a, b| a.weight_in_kg.cmp(&b.weight_in_kg))
        .expect("We are sure there is at least one value, so it should not be possible");
    let min_weight_entry = sorted_weights
        .iter()
        .min_by(|a, b| a.weight_in_kg.cmp(&b.weight_in_kg))
        .expect("We are sure there is at least one value, so it should not be possible");

    Some(UserWeightInfosResponse {
        last_3_weights,
        average_weight,
        number_of_weight_entries,
        average_weight_last_7_days,
        number_of_weight_entries_last_7_days,
        average_weight_last_30_days,
        number_of_weight_entries_last_30_days,
        max_weight: max_weight_entry.weight_in_kg,
        max_weight_date: max_weight_entry
            .recorded_at
            .and_hms_opt(0, 0, 0)
            .expect("Valid time should be constructible")
            .and_local_timezone(chrono::FixedOffset::east_opt(0).unwrap())
            .unwrap(),
        min_weight: min_weight_entry.weight_in_kg,
        min_weight_date: min_weight_entry
            .recorded_at
            .and_hms_opt(0, 0, 0)
            .expect("Valid time should be constructible")
            .and_local_timezone(chrono::FixedOffset::east_opt(0).unwrap())
            .unwrap(),
    })
}
