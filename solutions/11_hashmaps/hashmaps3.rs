// Foi fornecida uma lista de placares de futebol, um por linha. Cada linha tem
// o formato "<team_1_name>,<team_2_name>,<team_1_goals>,<team_2_goals>"
// Exemplo: "England,France,4,2" (England marcou 4 gols e France marcou 2).
//
// Você deve montar uma tabela com o nome de cada time, o total
// de gols marcados e o total de gols que o time
// sofreu.

use std::collections::HashMap;

// Estrutura para armazenar as estatísticas de gols de um time.
#[derive(Default)]
struct TeamScores {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_score_table(results: &str) -> HashMap<&str, TeamScores> {
    // O nome do time é a chave, e a struct associada é o valor.
    let mut scores = HashMap::<&str, TeamScores>::new();

    for line in results.lines() {
        let mut split_iterator = line.split(',');
        // OBSERVAÇÃO: usamos `unwrap` porque ainda não estudamos tratamento de erros.
        let team_1_name = split_iterator.next().unwrap();
        let team_2_name = split_iterator.next().unwrap();
        let team_1_score: u8 = split_iterator.next().unwrap().parse().unwrap();
        let team_2_score: u8 = split_iterator.next().unwrap().parse().unwrap();

        // Insira o valor padrão, com zeros, se o time ainda não existir.
        let team_1 = scores.entry(team_1_name).or_default();
        // Atualize os valores.
        team_1.goals_scored += team_1_score;
        team_1.goals_conceded += team_2_score;

        // Faça o mesmo para o segundo time.
        let team_2 = scores.entry(team_2_name).or_default();
        team_2.goals_scored += team_2_score;
        team_2.goals_conceded += team_1_score;
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

        assert!(
            ["England", "France", "Germany", "Italy", "Poland", "Spain"]
                .into_iter()
                .all(|team_name| scores.contains_key(team_name))
        );
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
