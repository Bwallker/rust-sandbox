use ndarray::{ArrayD, IxDyn};

pub(crate) fn dynamic_time_warp(s: &[i32], q: &[i32]) -> ArrayD<i32> {
    let mut dtw = ArrayD::zeros(IxDyn(&[s.len() + 1, q.len() + 1]));

    for i in 1..s.len() + 1 {
        dtw[[i, 0]] = i32::MAX;
    }
    for i in 1..q.len() + 1 {
        dtw[[0, i]] = i32::MAX;
    }
    dtw[[0, 0]] = 0;

    for i in 1..s.len() + 1 {
        for j in 1..q.len() + 1 {
            let cost = (s[i - 1] - q[j - 1]).abs();
            let min = i32::min(
                i32::min(dtw[[i - 1, j]], dtw[[i, j - 1]]),
                dtw[[i - 1, j - 1]],
            );
            if min == dtw[[i - 1, j - 1]] {
                println!("Found a match at i: {}, j: {}", i, j);
            }
            dtw[[i, j]] = cost + min;
        }
    }

    dtw
}
