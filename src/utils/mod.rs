use pulldown_cmark::{html, Options, Parser, OPTION_ENABLE_TABLES};
use timeago;
use chrono::{Datelike, Timelike, NaiveDateTime};
use ammonia::clean;
use comrak::{markdown_to_html, ComrakOptions};
use std::collections::btree_map::BTreeMap;

pub mod schema;
pub mod error;
pub mod token;
pub mod jwt;
pub mod state;

pub fn markdown2html(content: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(OPTION_ENABLE_TABLES);
    let mut capacity = String::with_capacity(content.len() * 3 / 2);
    let result = Parser::new_ext(content, opts);
    html::push_html(&mut capacity, result);
    capacity
}
#[inline]
pub fn markdown_render(md: &str) -> String {
    let option = ComrakOptions {
        ext_strikethrough: true,
        ext_table: true,
        ext_tasklist: true,
        ext_superscript: true,
        ..ComrakOptions::default()
    };
    clean(&markdown_to_html(md, &option))
}
pub fn time(end: NaiveDateTime, begin: NaiveDateTime) -> String {

    let end_time = ((end.year() as u64)* 365*24*3600) + ((end.month() as u64) *30*24*3600) + ((end.day() as u64) *24*3600)
                 + ((end.hour() as u64) *3600) + ((end.minute() as u64)*60) + (end.second() as u64 );
    let begin_time = ((begin.year() as u64)* 365*24*3600) + ((begin.month() as u64) *30*24*3600) + ((begin.day() as u64)*24*3600)
                   + ((begin.hour() as u64)*3600) + ((begin.minute() as u64)*60) + (begin.second() as u64) ;
    let secs = end_time - begin_time;
    
    let timeago = timeago::Formatter::new();
    let result = ::std::time::Duration::from_secs(secs);
    timeago.convert(result)
}

pub fn order_vec(vec: Vec<i32>) -> Vec<i32> {
        let mut theme_ids_count = BTreeMap::default();
        for &i in &vec {
            *theme_ids_count.entry(i).or_insert(0) += 1;
        }
        let mut theme_ids_result:Vec<_> = theme_ids_count.into_iter().collect();
        theme_ids_result.sort_by_key(|&(c,_)|::std::cmp::Reverse(c));
        theme_ids_result.sort_by_key(|&(_,c)|::std::cmp::Reverse(c));
        theme_ids_result.into_iter().map(|(i,_)|i).collect()

}

pub fn order_str_vec(vec: Vec<&str>) -> Vec<&str> {
        let mut theme_ids_count = BTreeMap::default();
        for &i in &vec {
            *theme_ids_count.entry(i).or_insert(0) += 1;
        }
        let mut theme_ids_result:Vec<_> = theme_ids_count.into_iter().collect();
        theme_ids_result.sort_by_key(|&(_,c)|::std::cmp::Reverse(c));
        theme_ids_result.into_iter().map(|(i,_)|i).collect()

}