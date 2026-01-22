pub struct Solution;

#[allow(dead_code)]
impl Solution {
  // Intuitively, we want to hire the paid painter for the walls that take the lowest costs and longest times to paint.
  // The longer the wall takes for the paid painer to paint, the more time we have for the free painter to paint other walls.
  //
  // If the wall i take cost[i] and time[i] to paint by the paid painter,
  // While the paid painter is doing his job, the free painter can paint time[i] walls for free.
  // So, overall, at the wall i, if we hire the paid painter:
  // pay the cost[i] price, we can paint 1 + time[i] walls (1 done by paid painter, time[i] done by free painter).
  //
  // The number of walls need to be painted is fixed at N.
  // The problem becomes: Find the set of walls I to hire the paid painter
  // so that the sum of (1 + time[i]) is >= N, and the sum of cost[i] is minimized.
  //
  // Thinking in reverse, we can reframe the problem like this:
  // When the paid painter paints the walls in set I,
  // the free painter paints the walls not in I, call it set J = ^I.
  // Requirement: sum of (1 + time[i]) for i in I >= N
  //   -> becomes: sum of (1 + time[j]) for j in J <= total_time + N - N = total_time
  // Requirement: minimize sum of cost[i] for i in I
  //   -> becomes: maximize sum of cost[j] for j in J
  // This, becomes a Classic 0-1 Knapsack Problem:
  // Given:
  //   weights = [1 + time[n] for n in N]
  //   prices = [cost[n] for n in N]
  //   capacity = sum(time[n] for n in N)
  // Find the set J that maximizes sum of prices[j] while keeping sum of weights[j] <= capacity.
  // The answer to the original problem is then: total_cost - max_sum_of_prices_in_J

  pub fn paint_walls(cost: Vec<i32>, time: Vec<i32>) -> i32 {
    let w = time.iter().sum::<i32>();
    let sum_cost = cost.iter().sum::<i32>();
    let ptime: Vec<i32> = time.iter().map(|&x| x + 1).collect();
    sum_cost - BinKnapsack::max_bottom_up(w, ptime, cost)
  }
}

pub struct BinKnapsack;

#[allow(dead_code)]
impl BinKnapsack {
  pub fn max_bottom_up(knapsack_cap: i32, weights: Vec<i32>, prices: Vec<i32>) -> i32 {
    let n = weights.len();
    if n <= 0 {
      return 0;
    }

    let cap: usize = knapsack_cap as usize;
    let mut r1: Vec<i32> = vec![0; cap + 1];
    let mut r2: Vec<i32> = vec![0; cap + 1];
    let mut cr = &mut r1;
    let mut pr = &mut r2;

    for i in 1..=n {
      let weight = weights[i - 1] as usize;
      let price = prices[i - 1];
      for w in 0..=cap {
        let max_if_dont_take = pr[w];
        let mut max_if_take: i32 = 0;
        if w >= weight {
          // Can take the item with current capacity w
          max_if_take = pr[w - weight] + price;
        }
        cr[w] = max_if_dont_take.max(max_if_take);
      }

      std::mem::swap(&mut cr, &mut pr);
    }

    pr[cap]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_paint_walls() {
    let cost = vec![1, 2, 3, 2];
    let time = vec![1, 2, 3, 2];
    let result = Solution::paint_walls(cost, time);
    assert_eq!(result, 3);

    let cost = vec![2, 3, 4, 2];
    let time = vec![1, 1, 1, 1];
    let result = Solution::paint_walls(cost, time);
    assert_eq!(result, 4);
  }
}
