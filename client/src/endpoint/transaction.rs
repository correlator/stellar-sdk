//! Contains endpoints for transactions and related information.
use error::Result;
use std::str::FromStr;
use stellar_resources::Transaction;
use super::{Body, EndPoint, Order, Records};
use http::{Request, Uri};
pub use super::account::Transactions as ForAccount;

/// Represents the all transactions end point for the stellar horizon server. The endpoint
/// will return all transactions filtered by cursor, order and limit
///
/// <https://www.stellar.org/developers/horizon/reference/endpoints/transactions-all.html>
///
/// ## Example
///
/// ```
/// use stellar_client::sync::Client;
/// use stellar_client::endpoint::transaction;
///
/// let client      = Client::horizon_test().unwrap();
/// let endpoint    = transaction::All::default();
/// let records     = client.request(endpoint).unwrap();
/// #
/// # assert!(records.records().len() > 0);
/// ```
#[derive(Debug, Default)]
pub struct All {
    cursor: Option<String>,
    order: Option<Order>,
    limit: Option<u32>,
}

impl All {
    /// Starts the page of results at a given cursor
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::transaction;
    /// # use stellar_client::endpoint::Order;
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// #
    /// # // grab first page and extract cursor
    /// # let endpoint      = transaction::All::default();
    /// # let first_page    = client.request(endpoint).unwrap();
    /// # let cursor        = first_page.next_cursor();
    /// #
    /// let endpoint    = transaction::All::default().cursor(cursor);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// # assert_ne!(records.next_cursor(), cursor);
    /// ```
    pub fn cursor(mut self, cursor: &str) -> Self {
        self.cursor = Some(cursor.to_string());
        self
    }

    /// Fetches all records with a given limit
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::transaction;
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = transaction::All::default().limit(1);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert_eq!(records.records().len(), 1);
    /// ```
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Fetches all records in a set order, either ascending or descending.
    ///
    /// ## Example
    ///
    /// ```
    /// use stellar_client::sync::Client;
    /// use stellar_client::endpoint::{transaction, Order};
    ///
    /// let client      = Client::horizon_test().unwrap();
    /// let endpoint    = transaction::All::default().order(Order::Asc);
    /// let records     = client.request(endpoint).unwrap();
    /// #
    /// # assert!(records.records().len() > 0);
    /// ```
    pub fn order(mut self, order: Order) -> Self {
        self.order = Some(order);
        self
    }

    fn has_query(&self) -> bool {
        self.order.is_some() || self.cursor.is_some() || self.limit.is_some()
    }
}

impl EndPoint for All {
    type Response = Records<Transaction>;

    fn into_request(self, host: &str) -> Result<Request<Body>> {
        let mut uri = format!("{}/transactions", host);
        if self.has_query() {
            uri.push_str("?");

            if let Some(cursor) = self.cursor {
                uri.push_str(&format!("cursor={}&", cursor));
            }

            if let Some(order) = self.order {
                uri.push_str(&format!("order={}&", order.to_param()));
            }

            if let Some(limit) = self.limit {
                uri.push_str(&format!("limit={}", limit));
            }
        }

        let uri = Uri::from_str(&uri)?;
        let request = Request::get(uri).body(Body::None)?;
        Ok(request)
    }
}

#[cfg(test)]
mod all_transactions_test {
    use super::*;

    #[test]
    fn it_leaves_off_the_params_if_not_specified() {
        let ep = All::default();
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions");
        assert_eq!(req.uri().query(), None);
    }

    #[test]
    fn it_puts_the_query_params_on_the_uri() {
        let ep = All::default()
            .cursor("CURSOR")
            .order(Order::Desc)
            .limit(123);
        let req = ep.into_request("https://www.google.com").unwrap();
        assert_eq!(req.uri().path(), "/transactions");
        assert_eq!(
            req.uri().query(),
            Some("cursor=CURSOR&order=desc&limit=123")
        );
    }
}
