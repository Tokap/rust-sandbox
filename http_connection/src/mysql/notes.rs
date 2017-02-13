// Can Call on a Pool to get a connection:
fn get_conn(&self) -> MyResult<PooledConn>
//
// Gives you a PooledConn.
// Pool will check that connection is alive via
// Conn::ping and will call Conn::reset if necessary.

// Building a simple non-generic connection:
fn build_pool() -> Pool {
    let mut builder = OptsBuilder::new();
    builder
        .ip_or_hostname(Some("127.0.0.1"))
        .tcp_port(3306)
        .db_name(Some("ip_brolytics"))
        .user(Some("root"));
    let pool = Pool::new(builder);
    pool.unwrap()
}
// NOTE: Pools can be cloned with .clone() & debug is implemented


// Testing connection:
fn test_and_output_connection(p: Pool) -> () {
    match p.try_get_conn(50000) {
        Ok(rez) => println!("Okay! {:?}", rez),
        Err(e) => println!("Bad news: {:?}", e),
    }
}
