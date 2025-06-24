use sqlx::SqlitePool;

pub async fn update_spellfix_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM BookSpellfix").execute(&mut *tx).await?;
    
    sqlx::query("
        INSERT INTO BookSpellfix (word, rank)
        SELECT term, doc FROM BookFtsVocab 
        ").execute(&mut *tx).await?;

    tx.commit().await
}

pub struct SpellfixCandidates {
    candidates: Vec<Vec<Candidate>>,
    exhausted: bool
}

impl SpellfixCandidates {
    fn new(candidates: Vec<Vec<Candidate>>) -> Self {
        Self {
            candidates,
            exhausted: false
        } 
    }

    pub fn get_top_candidate(&self) -> String {
        self.candidates.iter()
            .map(|c| match c.first() {
                Some(c) => c.word.clone(), None => String::new()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Iterator for SpellfixCandidates {
    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            return None;
        }
        
        let mut top_candidates: Vec<(Candidate, usize)> = vec![];

        let mut composed_str = String::new();
        for (i, word_candidates) in self.candidates.iter().enumerate() {
            let candidate = word_candidates.first().unwrap();
            
            composed_str.push_str(&candidate.word);

            if word_candidates.len() > 1 {
                top_candidates.push((candidate.clone(), i));
            }
        }
        
        top_candidates.sort_by_key(|k| k.0.score);

        match top_candidates.first() {
            Some((_, i)) => { self.candidates[*i].pop(); },
            None => { self.exhausted = true; }
        };

        Some(composed_str)
    }

    type Item = String;

}

pub async fn get_spelling_candidates(pool: &SqlitePool, string: &str, max_candidates: u8) -> Result<Option<SpellfixCandidates>, sqlx::Error> {
    let words = string.split_whitespace();
    
    let mut corrected_words = vec![];

    let mut tx = pool.begin().await?;
    for word in words {
        let candidates: Vec<Candidate> = sqlx::query_as("
            SELECT word, score FROM BookSpellfix WHERE word MATCH ? AND top=?
            ").bind(format!("{}*", word)).bind(max_candidates.min(20)).fetch_all(&mut *tx).await?;
        if candidates.len() > 0 {
            corrected_words.push(candidates);
        }
    }
    tx.commit().await?;

    if corrected_words.len() > 0 {
        return Ok(Some(SpellfixCandidates::new(corrected_words)))
    }
    Ok(None)
}

#[derive(sqlx::FromRow, Clone)]
struct Candidate {
    word: String,
    score: i32,
}

impl Ord for Candidate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Candidate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score.partial_cmp(&other.score) 
    } 
}

impl Eq for Candidate {}
impl PartialEq for Candidate {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}
