use crate::errors::*;

const RATER_BETA: f64 = 25.0 / 6.0;

pub fn get_rating_change<T>(
    players: Vec<(T, u32, bbt::Rating)>,
) -> Result<Vec<(T, u32, bbt::Rating, bbt::Rating)>> {
    let rater = bbt::Rater::new(RATER_BETA);
    let new_ratings = rater.update_ratings(
        players
            .iter()
            .map(|(_, _, r)| vec![r.clone()])
            .collect::<Vec<_>>(),
        players
            .iter()
            .map(|(_, s, _)| 2 - *s as usize)
            .collect::<Vec<_>>(),
    )?;

    Ok(players
        .into_iter()
        .zip(new_ratings.into_iter())
        .map(|((p, s, r), nr)| (p, s, r, nr[0].clone()))
        .collect::<Vec<_>>())
}
