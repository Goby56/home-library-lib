use std::collections::HashMap;
use std::vec;

use uuid::Uuid;
use levenshtein::levenshtein;
use sorted_vec::partial::ReverseSortedVec;

use crate::err::{BookBorrowingError, ListBorrowsError};
use crate::searching::{Comparison, SearchResult};

use super::bk::BkTree;
use super::data::Book;
use super::serialize::Serializer;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

pub struct Library {
    pub bk_tree: Option<BkTree>,
    pub books: Vec<Book>
}

impl Library { 
    pub fn from(books: Vec<Book>) -> Library {
        Library { bk_tree: None, books }
    }

    pub fn add_book(&mut self, book: Book) {
        let index = (self.books.len()) as u32;
        if let Some(tree) = self.bk_tree.as_mut() {
            tree.add_node(book.metadata.title.clone(), vec![index]);
            tree.add_node(format!("@{}", book.metadata.author), vec![index]);
        }
        self.books.push(book);
    }

    pub fn search(&self, query: &str, limit: Option<usize>, year_expr: Option<String>) -> Vec<SearchResult>  {
        let mut search_results: ReverseSortedVec<SearchResult> = ReverseSortedVec::new();

        let mut comparisons: Vec<Comparison> = vec![];
        if let Some(year_comp) = year_expr {
            comparisons.extend(Comparison::from_string(&year_comp));
        }

        let matcher = SkimMatcherV2::default();
        for book in &self.books {
            if !Comparison::batch_compare(&comparisons, book.metadata.pub_date as i32) {
                continue;
            }
            if let Some(score) = matcher.fuzzy_match(&book.serialize(), query) {
                search_results.insert(SearchResult{book: book.clone(), score}); 
            }
        }
        let cutoff = limit.unwrap_or(search_results.len());
        if search_results.len() < cutoff {
            // book_results.extend(self.bk_search(query));
        }
        return search_results[..cutoff].to_vec();
    }

    fn bk_search(&mut self, query: &str) -> Vec<(&Book, u16)> {
        if let Some(tree) = self.bk_tree.as_mut() {
            let mut books_and_distance = vec![];
            for result in tree.search(query) {
                for book_ref in result.contents.get_refs() {
                    let b = self.books.get(book_ref as usize);
                    if let Some(b) = b {
                        books_and_distance.push((b, result.distance));
                    }
                }
            }
            return books_and_distance;
        }
        return vec![];
    }

    pub fn modify_borrow(&mut self, user: Option<String>, uuid: Uuid) -> Result<Book, BookBorrowingError> {
        for i in 0..(self.books.len()) {
            let book = &mut self.books[i];
            if !book.uuid.eq(&uuid) {
                continue;
            }
            match &user {
                Some(new_owner) => book.borrow(new_owner)?,
                None => book.return_()?
            }
            return Ok(book.clone())
        }
        Err(BookBorrowingError { book_title: None, borrower: None, uuid: uuid.to_string() })
    }

    pub fn list_borrows(&self, borrower: &str) -> Result<Vec<&Book>, ListBorrowsError> {
        let lc_input = &borrower.to_lowercase();
        let mut search_result: HashMap<String, Vec<&Book>> = HashMap::new();
        for i in 0..(self.books.len()) {
            let book = &self.books[i];
            if let Some(book_borrower) = &book.borrower {
                if book_borrower.eq(&borrower) {
                    if let Some(books) = search_result.get_mut(borrower) {
                        books.push(book);
                    }
                }
            }
        }
        let mut best_match = "";
        let mut shortest_dist = 10;
        for user in search_result.keys() {
            let dist = levenshtein(lc_input, &user.to_lowercase());
            if dist < shortest_dist {
                shortest_dist = dist;
                best_match = user;
            }
        }
        if &best_match.to_lowercase() == lc_input {
            if let Some(books) = search_result.get(best_match) {
                return Ok(books.to_vec());
            }
        }
        match best_match {
            "" => return Err(ListBorrowsError { 
                input_borrower: borrower.to_string(), 
                found_borrower: None
            }),
            _ => return Err(ListBorrowsError { 
                input_borrower: borrower.to_string(), 
                found_borrower: Some(best_match.to_string())
            })
        };
    }
}
