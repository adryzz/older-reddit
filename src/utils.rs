pub fn get_reddit_domain() -> url_builder::URLBuilder {
    let mut a = url_builder::URLBuilder::new();

    a.set_protocol("https").set_host("pay.reddit.com");

    a
}
