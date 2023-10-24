// pub fn deserialize_opt_prune_mode_with_min_blocks<
//     'de,
//     const MIN_BLOCKS: u64,
//     D: Deserializer<'de>,
// >(
//     deserializer: D,
// ) -> Result<Option<PruneMode>, D::Error> {
//     let prune_mode = Option::<PruneMode>::deserialize(deserializer)?;

//     match prune_mode {
//         Some(PruneMode::Full) if MIN_BLOCKS > 0 => {
//             Err(serde::de::Error::invalid_value(
//                 serde::de::Unexpected::Str("full"),
//                 // This message should have "expected" wording
//                 &format!("prune mode that leaves at least {MIN_BLOCKS} blocks in the database")
//                     .as_str(),
//             ))
//         }
//         Some(PruneMode::Distance(distance)) if distance < MIN_BLOCKS => {
//             Err(serde::de::Error::invalid_value(
//                 serde::de::Unexpected::Unsigned(distance),
//                 // This message should have "expected" wording
//                 &format!("prune mode that leaves at least {MIN_BLOCKS} blocks in the database")
//                     .as_str(),
//             ))
//         }
//         _ => Ok(prune_mode),
//     }
// }

// #[cfg(test)]
// mod test {
//     use crate::PruneMode;
//     use assert_matches::assert_matches;
//     use serde::Deserialize;

//     #[test]
//     fn deserialize_opt_prune_mode_with_min_blocks() {
//         #[derive(Debug, Deserialize, PartialEq, Eq)]
//         struct V(
//             #[serde(
//                 deserialize_with = "super::deserialize_opt_prune_mode_with_min_blocks::<10, _>"
//             )]
//             Option<PruneMode>,
//         );

//         assert!(serde_json::from_str::<V>(r#"{"distance": 10}"#).is_ok());
//         assert_matches!(
//             serde_json::from_str::<V>(r#"{"distance": 9}"#),
//             Err(err) if err.to_string() == "invalid value: integer `9`, expected prune mode that leaves at least 10 blocks in the database"
//         );

//         assert_matches!(
//             serde_json::from_str::<V>(r#""full""#),
//             Err(err) if err.to_string() == "invalid value: string \"full\", expected prune mode that leaves at least 10 blocks in the database"
//         );
//     }
// }
