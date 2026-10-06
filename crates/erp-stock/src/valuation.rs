use crate::error::StockError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FifoBatch {
    pub qty: Decimal,
    pub rate: Decimal,
}

pub struct FifoEngine;

impl FifoEngine {
    /// Consumes qty from FIFO queue and returns (consumed_cost, new_queue)
    pub fn consume(
        mut queue: Vec<FifoBatch>,
        mut qty_to_consume: Decimal,
    ) -> Result<(Decimal, Vec<FifoBatch>), StockError> {
        let mut total_cost = Decimal::ZERO;
        let mut new_queue = Vec::new();

        for mut batch in queue.drain(..) {
            if qty_to_consume <= Decimal::ZERO {
                new_queue.push(batch);
                continue;
            }

            if batch.qty <= qty_to_consume {
                // Consume entire batch
                total_cost += batch.qty * batch.rate;
                qty_to_consume -= batch.qty;
            } else {
                // Consume partial batch
                total_cost += qty_to_consume * batch.rate;
                batch.qty -= qty_to_consume;
                qty_to_consume = Decimal::ZERO;
                new_queue.push(batch);
            }
        }

        if qty_to_consume > Decimal::ZERO {
            return Err(StockError::NegativeStock {
                item_code: "FIFO".into(),
                warehouse_id: "FIFO".into(),
                available: Decimal::ZERO,
                requested: qty_to_consume,
            });
        }

        Ok((total_cost, new_queue))
    }

    /// Adds incoming stock to the FIFO queue
    pub fn receive(mut queue: Vec<FifoBatch>, qty: Decimal, rate: Decimal) -> Vec<FifoBatch> {
        if qty > Decimal::ZERO {
            queue.push(FifoBatch { qty, rate });
        }
        queue
    }
}

pub struct MovingAverageEngine;

impl MovingAverageEngine {
    /// Calculates new valuation rate upon inward stock movement:
    /// New Rate = ((Current Qty * Current Rate) + (Incoming Qty * Incoming Rate)) / (Current Qty + Incoming Qty)
    pub fn calculate_incoming_rate(
        current_qty: Decimal,
        current_rate: Decimal,
        incoming_qty: Decimal,
        incoming_rate: Decimal,
    ) -> Decimal {
        if incoming_qty <= Decimal::ZERO {
            return current_rate;
        }

        let current_val = current_qty * current_rate;
        let incoming_val = incoming_qty * incoming_rate;
        let total_qty = current_qty + incoming_qty;

        if total_qty <= Decimal::ZERO {
            return Decimal::ZERO;
        }

        (current_val + incoming_val) / total_qty
    }

    /// Calculates stock value difference for outward movement
    pub fn calculate_outgoing_difference(outgoing_qty: Decimal, current_rate: Decimal) -> Decimal {
        outgoing_qty * current_rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_consumption() {
        let mut queue = Vec::new();
        // Receive 10 units @ $10.00
        queue = FifoEngine::receive(queue, Decimal::new(10, 0), Decimal::new(10, 0));
        // Receive 20 units @ $15.00
        queue = FifoEngine::receive(queue, Decimal::new(20, 0), Decimal::new(15, 0));

        // Consume 15 units: (10 * 10) + (5 * 15) = 100 + 75 = 175
        let (cost, remaining_queue) = FifoEngine::consume(queue, Decimal::new(15, 0)).unwrap();
        assert_eq!(cost, Decimal::new(175, 0));
        assert_eq!(remaining_queue.len(), 1);
        assert_eq!(remaining_queue[0].qty, Decimal::new(15, 0));
        assert_eq!(remaining_queue[0].rate, Decimal::new(15, 0));
    }

    #[test]
    fn test_moving_average_calculation() {
        // Initially 100 units @ $2.00
        let current_qty = Decimal::new(100, 0);
        let current_rate = Decimal::new(2, 0);

        // Receive 50 units @ $3.50
        let incoming_qty = Decimal::new(50, 0);
        let incoming_rate = Decimal::new(35, 1);

        // Total value = 200 + 175 = 375. Total qty = 150. New rate = 375 / 150 = 2.50
        let new_rate = MovingAverageEngine::calculate_incoming_rate(
            current_qty,
            current_rate,
            incoming_qty,
            incoming_rate,
        );
        assert_eq!(new_rate, Decimal::new(25, 1));
    }
}
