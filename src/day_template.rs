use crate::util::Errors::NoImplementationError;
use crate::util::{load_from, Errors};
use crate::Day;
use core::str::Lines;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct DayTT {}

impl Day for DayTT {
    fn part_1(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn part_2(&self) -> Result<String, Errors> {
        Err(NoImplementationError)
    }

    fn create_day() -> Box<dyn Day> where Self: Sized {
        Box::new(DayTT {})
    }
}

#[cfg(test)]
mod tests {

}