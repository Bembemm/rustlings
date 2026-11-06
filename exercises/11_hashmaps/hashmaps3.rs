// Você recebeu uma lista de placares de futebol, um por linha, no formato
// "<time_1>,<time_2>,<gols_time_1>,<gols_time_2>".
// Exemplo: "England,France,4,2" (a Inglaterra marcou 4 gols e a França, 2).
//
// Monte uma tabela que guarde o nome de cada equipe, o total de gols
// marcados e o total de gols sofridos por ela.

use std::collections::HashMap;

// Estrutura para armazenar as estatísticas de gols de um time.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_score_table(results: &str) -> HashMap<&str, TeamScores> {
    // O nome do time é a chave; a estrutura associada é o valor.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // OBSERVAÇÃO: Usamos `unwrap` porque ainda não estudamos tratamento de erros.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // TODO: Preencha a tabela de placares com os dados extraídos.
        // Lembre-se de que os gols marcados pelo time 1 foram sofridos pelo
        // time 2. Da mesma forma, os gols do time 2 foram sofridos pelo time 1.
    }

    scores
}

fn main() {
    // Se quiser, você pode fazer experimentos aqui.
}

#[cfg(test)]
mod tests {
    use super::*;

    const RESULTS: &str = "England,France,4,2
France,Italy,3,1
Poland,Spain,2,0
Germany,England,2,1
England,Spain,1,0";

    #[test]
    fn build_scores() {
        let scores = build_score_table(RESULTS);

        assert!(["England", "France", "Germany", "Italy", "Poland", "Spain"]
            .into_iter()
            .all(|team_name| scores.contains_key(team_name)));
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 6);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_score_table(RESULTS);
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 3);
    }
}
