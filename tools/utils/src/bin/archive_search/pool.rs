pub fn run_ranked<F, G>(
    items: &[String],
    jobs: usize,
    run: F,
    mut proceed: G,
) -> Vec<(String, Vec<String>)>
where
    F: Fn(&str) -> Vec<String> + Sync,
    G: FnMut(&str, &[String]) -> bool,
{
    let mut out = Vec::new();
    for chunk in items.chunks(jobs) {
        let mut results: Vec<Vec<String>> = Vec::with_capacity(chunk.len());
        std::thread::scope(|scope| {
            let run = &run;
            let handles: Vec<_> = chunk
                .iter()
                .map(|item| scope.spawn(move || run(item)))
                .collect();
            for handle in handles {
                results.push(handle.join().expect("arm worker panicked"));
            }
        });
        for (item, lines) in chunk.iter().zip(results.into_iter()) {
            let keep_going = proceed(item, &lines);
            out.push((item.clone(), lines));
            if !keep_going {
                return out;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn one_job_and_four_jobs_keep_the_rank_order_and_results() {
        let items: Vec<String> = (0..9).map(|n| format!("arm{n}")).collect();
        let run = |item: &str| {
            let n: u64 = item.trim_start_matches("arm").parse().unwrap_or(0);
            std::thread::sleep(Duration::from_millis((9 - n) * 3));
            vec![format!("url https://example.org/{item}")]
        };
        let serial = run_ranked(&items, 1, run, |_, _| true);
        let parallel = run_ranked(&items, 4, run, |_, _| true);
        assert_eq!(serial, parallel);
        let order: Vec<&str> = parallel.iter().map(|(item, _)| item.as_str()).collect();
        assert_eq!(
            order,
            vec![
                "arm0", "arm1", "arm2", "arm3", "arm4", "arm5", "arm6", "arm7", "arm8"
            ]
        );
    }

    #[test]
    fn stop_predicate_commits_the_same_prefix_for_any_job_count() {
        let items: Vec<String> = (0..9).map(|n| format!("arm{n}")).collect();
        let run = |item: &str| vec![format!("url https://example.org/{item}")];
        let serial = run_ranked(&items, 1, run, |item, _| item != "arm3");
        let parallel = run_ranked(&items, 4, run, |item, _| item != "arm3");
        assert_eq!(serial, parallel);
        assert_eq!(serial.len(), 4);
    }

    #[test]
    fn a_single_job_keeps_the_rank_order() {
        let items: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let out = run_ranked(&items, 1, |item| vec![item.to_string()], |_, _| true);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].0, "a");
        assert_eq!(out[1].0, "b");
    }
}
