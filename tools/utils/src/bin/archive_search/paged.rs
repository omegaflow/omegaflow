pub const DEFAULT_PAGE_BUDGET: usize = 10;

pub enum StopReason {
    Empty,
    Budget,
}

impl StopReason {
    pub fn label(&self) -> &'static str {
        match self {
            StopReason::Empty => "empty",
            StopReason::Budget => "budget",
        }
    }
}

pub fn follow_pages<F: FnMut(usize) -> (Vec<String>, bool)>(
    budget: usize,
    mut page: F,
) -> (Vec<String>, StopReason) {
    let mut lines: Vec<String> = Vec::new();
    for index in 0..budget {
        let (mut page_lines, has_more) = page(index);
        lines.append(&mut page_lines);
        if !has_more {
            return (lines, StopReason::Empty);
        }
    }
    (lines, StopReason::Budget)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_single_page_that_ends_is_empty() {
        let (lines, reason) = follow_pages(10, |_| (vec!["a".to_string(), "b".to_string()], false));
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(reason.label(), "empty");
    }

    #[test]
    fn two_pages_end_on_the_second() {
        let (lines, reason) = follow_pages(10, |index| match index {
            0 => (vec!["a".to_string()], true),
            _ => (vec!["b".to_string()], false),
        });
        assert_eq!(lines, vec!["a".to_string(), "b".to_string()]);
        assert_eq!(reason.label(), "empty");
    }

    #[test]
    fn a_full_budget_truncates() {
        let (lines, reason) = follow_pages(3, |index| (vec![format!("page{}", index)], true));
        assert_eq!(
            lines,
            vec![
                "page0".to_string(),
                "page1".to_string(),
                "page2".to_string()
            ]
        );
        assert_eq!(reason.label(), "budget");
    }

    #[test]
    fn zero_budget_returns_budget_without_a_page() {
        let mut calls = 0;
        let (lines, reason) = follow_pages(0, |_| {
            calls += 1;
            (vec!["never".to_string()], true)
        });
        assert!(lines.is_empty());
        assert_eq!(calls, 0);
        assert_eq!(reason.label(), "budget");
    }
}
