use crate::trade::ContractSymbolTrade;
use crate::trade::DirectionTrade;
pub mod handler;
pub mod subscriber;

#[derive(Debug, Clone)]
pub enum PositionStateTrade {
    /// The position is open
    ///
    /// Open in the sense, that there is an active position that is being rolled-over.
    /// Note that a "closed" position does not exist, but is just removed.
    /// During the process of getting closed (after creating the counter-order that will wipe out
    /// the position), the position is in state "Closing".
    ///
    /// Transitions:
    /// Open->Closing
    Open,
    /// The position is in the process of being closed
    ///
    /// The user has created an order that will wipe out the position.
    /// Once this order has been filled the "closed" the position is not shown in the user
    /// interface, so we don't have a "closed" state because no position data will be provided to
    /// the user interface.
    Closing,
}

#[derive(Debug, Clone)]
pub struct PositionTrade {
    pub leverage: f64,
    pub quantity: f64,
    pub contract_symbol: ContractSymbolTrade,
    pub direction: DirectionTrade,
    pub average_entry_price: f64,
    pub liquidation_price: f64,
    /// The unrealized PL can be positive or negative
    pub unrealized_pnl: i64,
    pub position_state: PositionStateTrade,
}

impl Default for PositionTrade {
    fn default() -> Self {
        PositionTrade {
            leverage: 0.0,
            quantity: 0.0,
            contract_symbol: ContractSymbolTrade::BtcUsd,
            direction: DirectionTrade::Long,
            average_entry_price: 0.0,
            liquidation_price: 0.0,
            unrealized_pnl: 0,
            position_state: PositionStateTrade::Open,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct TradeParams {
    // TODO Trade parameters needed to contact coordinator for trade execution
}