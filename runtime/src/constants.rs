pub mod currency {

	pub const MILLICENTS: u128 = 1_000_000_000;
	pub const CENTS: u128 = 1_000 * MILLICENTS;    // assume this is worth about a cent.
	pub const DOLLARS: u128 = 100 * CENTS;

	pub const fn deposit(items: u32, bytes: u32) -> u128 {
		items as Balance * 15 * CENTS + (bytes as u128) * 6 * CENTS
	}
}