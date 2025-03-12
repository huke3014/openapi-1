use longport_candlesticks::{
    markets::US, TRADE_SESSION_NORMAL, TRADE_SESSION_OVERNIGHT, TRADE_SESSION_POST,
    TRADE_SESSION_PRE,
};
use time::macros::datetime;

#[test]
fn us_trade_session() {
    let market = US;

    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 20:00:00 -5)),
        Some(TRADE_SESSION_OVERNIGHT)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 23:59:59 -5)),
        Some(TRADE_SESSION_OVERNIGHT)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-2 0:00:00 -5)),
        Some(TRADE_SESSION_OVERNIGHT)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-2 3:59:59 -5)),
        Some(TRADE_SESSION_OVERNIGHT)
    );

    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 4:00:00 -5)),
        Some(TRADE_SESSION_PRE)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 8:00:00 -5)),
        Some(TRADE_SESSION_PRE)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 9:29:59 -5)),
        Some(TRADE_SESSION_PRE)
    );

    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 9:30:00 -5)),
        Some(TRADE_SESSION_NORMAL)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 12:30:00 -5)),
        Some(TRADE_SESSION_NORMAL)
    );
    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 15:59:59 -5)),
        Some(TRADE_SESSION_NORMAL)
    );

    assert_eq!(
        market.candlestick_trade_session(datetime!(2024-1-1 16:00:00 -5)),
        Some(TRADE_SESSION_POST)
    );
}
