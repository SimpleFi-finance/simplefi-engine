use interfaces::Result;
use primitives::{H256, Market, TokenMarkets};

/// Client trait for [Market]s
pub trait MarketProvider: Send + Sync {
    /// Creates a new market.
    fn create_market (&self, market: Market, market_address: H256) -> Result<()>;

    /// Deletes an existing market. IF the market does not exist, an Error Result will be thrown
    fn delete_market (&self, market_address: H256) -> Result<()>;

    /// Retrieves a single market
    fn get_market (&self, market_address: H256) -> Result<Option<Market>>;

    /// Updates an existing market. IF the market does not exist, an Error Result will be thrown
    fn update_market (&self, market_address: H256, updated_market: Market) -> Result<()>;

    fn add_to_token_markets (&self, market_address: H256, token_address: H256) -> Result<()>;
    fn get_token_markets(&self, token_address: H256) -> Result<Option<TokenMarkets>>;

}


